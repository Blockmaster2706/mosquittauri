use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, RecvTimeoutError, Sender},
        Arc,
    },
    thread::{spawn, JoinHandle},
    time::{Duration, Instant},
};

use anyhow::Result;
use rumqttc::{AsyncClient, MqttOptions, Publish, QoS};
use tauri::async_runtime as tk;

use super::MqttConnection;
use crate::{ipc::event::MqttSendEvent, model::Message};

pub struct MqttPool {
    options: MqttOptions,
    connections: Vec<MqttConnection>,
    topics: Vec<String>,
    running: Arc<AtomicBool>,
    msg_listener_handle: JoinHandle<()>,
    msg_sender_handle: JoinHandle<()>,
    publish_sender: Sender<Publish>,
    capacity: usize,
}

impl MqttPool {
    pub const MQTT_CLIENT_CAP: usize = 16;
    pub const SEND_QUEUE_LEN: usize = 16;
    pub const PACKET_QUEUE_LEN: usize = 96;
    pub const MSG_BATCH_QUEUE_LEN: usize = 16;

    #[must_use]
    pub fn new(options: MqttOptions) -> (Self, Sender<MqttSendEvent>, Receiver<Vec<Message>>) {
        let running = Arc::new(AtomicBool::new(true));
        let (msg_listener_sender, msg_listener_receiver) = channel();
        let (msg_sender_sender, msg_sender_receiver) = channel();
        let (msg_listener_handle, publish_sender) =
            Self::start_msg_listener(running.clone(), msg_listener_sender);
        let msg_sender_handle =
            Self::start_msg_sender(options.clone(), running.clone(), msg_sender_receiver);
        (
            Self {
                options,
                connections: Vec::new(),
                msg_listener_handle,
                msg_sender_handle,
                publish_sender,
                running,
                topics: Vec::new(),
                capacity: 10,
            },
            msg_sender_sender,
            msg_listener_receiver,
        )
    }
    pub fn add_subscriber(&mut self, topic: impl Into<String>) -> Result<()> {
        let (client, eventloop) = AsyncClient::new(self.options.clone(), Self::MQTT_CLIENT_CAP);
        tk::block_on(client.subscribe(topic, QoS::ExactlyOnce))?;
        let mqtt_connection =
            MqttConnection::new(eventloop, self.publish_sender.clone(), self.running.clone());
        self.connections.push(mqtt_connection);
        Ok(())
    }
    pub fn get_running_atomic(&self) -> Arc<AtomicBool> {
        self.running.clone()
    }
    pub fn disconnect(self) {
        self.running.store(false, Ordering::Relaxed);
        for conn in self.connections {
            conn.await_disconnect();
        }
        if let Err(e) = self.msg_listener_handle.join() {
            log::warn!("Failed to stop message listner: {e:?}")
        };
    }

    fn start_msg_sender(
        options: MqttOptions,
        running: Arc<AtomicBool>,
        receiver: Receiver<MqttSendEvent>,
    ) -> JoinHandle<()> {
        let (client, _eventloop) = AsyncClient::new(options, Self::MQTT_CLIENT_CAP);
        spawn(move || {
            while running.load(Ordering::Relaxed) {
                match receiver.recv_timeout(Duration::from_millis(1000)) {
                    Ok(msg) => {
                        if let Err(e) =
                            client.try_publish(msg.topic(), QoS::ExactlyOnce, true, msg.payload())
                        {
                            log::warn!("Failed to publish mqtt message: {e}")
                        }
                    }

                    Err(RecvTimeoutError::Timeout) => continue,

                    Err(RecvTimeoutError::Disconnected) => {
                        return;
                    }
                }
            }
            log::info!("Stopped message sender")
        })
    }

    fn start_msg_listener(
        running: Arc<AtomicBool>,
        message_sender: Sender<Vec<Message>>,
    ) -> (JoinHandle<()>, Sender<Publish>) {
        let (sender, receiver) = channel::<Publish>();
        let handle = spawn(move || {
            let start = Instant::now();
            let mut messages = Vec::new();
            while running.load(Ordering::Relaxed) {
                while start.elapsed().gt(&Duration::from_millis(1500)) {
                    let Ok(publish) = receiver.recv_timeout(Duration::from_millis(1000)) else {
                        continue;
                    };
                    let msg = match publish.try_into() {
                        Ok(msg) => msg,
                        Err(e) => {
                            log::warn!("Failed to parse message: {e}");
                            continue;
                        }
                    };
                    messages.push(msg);
                }
            }
            if let Err(e) = message_sender.send(messages) {
                log::warn!("Failed to send message to send channel: {e}");
            }
            log::info!("Stopped package listener")
        });
        (handle, sender)
    }
}
