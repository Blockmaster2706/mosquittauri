use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use anyhow::Result;
use rumqttc::{Client, MqttOptions, Publish, QoS};
use tauri::async_runtime::{block_on, channel, spawn, JoinHandle, Receiver, Sender};

use crate::{ipc::event::MqttSendEvent, model::Message};

use super::MqttConnection;

#[allow(dead_code)]
pub struct MqttPool {
    options: MqttOptions,
    connections: Vec<MqttConnection>,
    topics: Vec<String>,
    running: Arc<AtomicBool>,
    msg_listener_handle: JoinHandle<()>,
    publish_sender: Sender<Publish>,
    capacity: usize,
}

#[allow(dead_code)]
impl MqttPool {
    pub const SEND_QUEUE_LEN: usize = 16;
    pub const PACKET_QUEUE_LEN: usize = 96;
    pub const MSG_BATCH_QUEUE_LEN: usize = 16;

    pub fn new(options: MqttOptions, message_sender: Sender<Vec<Message>>) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let (msg_listener_handle, publish_sender) =
            Self::start_msg_listener(running.clone(), message_sender);
        Self {
            options,
            connections: Vec::new(),
            msg_listener_handle,
            publish_sender,
            running,
            topics: Vec::new(),
            capacity: 10,
        }
    }
    pub fn add_subscriber(&mut self, topic: impl Into<String>) -> Result<()> {
        let (client, connection) = Client::new(self.options.clone(), 10);
        client.subscribe(topic, QoS::ExactlyOnce)?;
        let mqtt_connection = MqttConnection::new(
            connection,
            self.publish_sender.clone(),
            self.running.clone(),
        );
        self.connections.push(mqtt_connection);
        Ok(())
    }
    pub fn get_running_atomic(&self) -> Arc<AtomicBool> {
        self.running.clone()
    }
    pub fn disconnect(self) {
        self.running.store(false, Ordering::Relaxed);
        let connections = self.connections.into_iter().map(|c| c.await_disconnect());
        block_on(async move {
            for conn in connections {
                conn.await;
            }
        });
        while self.msg_listener_handle.inner().is_finished() {}
    }

    fn start_send_queue(
        &mut self,
        running: Arc<AtomicBool>,
        mut receiver: Receiver<MqttSendEvent>,
    ) -> JoinHandle<()> {
        let (client, _connection) = Client::new(self.options.clone(), 10);
        spawn(async move {
            while running.load(Ordering::Relaxed) {
                let mut messages = Vec::new();
                receiver
                    .recv_many(&mut messages, Self::SEND_QUEUE_LEN)
                    .await;
                for msg in messages {
                    if let Err(e) =
                        client.publish(msg.topic(), QoS::ExactlyOnce, true, msg.payload())
                    {
                        log::error!("Failed to send message: {e}")
                    }
                }
            }
        })
    }

    fn start_msg_listener(
        running: Arc<AtomicBool>,
        message_sender: Sender<Vec<Message>>,
    ) -> (JoinHandle<()>, Sender<Publish>) {
        let (sender, mut receiver) = channel(96);
        let handle = spawn(async move {
            while running.load(Ordering::Relaxed) {
                let mut packets = Vec::new();
                receiver
                    .recv_many(&mut packets, Self::PACKET_QUEUE_LEN)
                    .await;
                #[allow(unused_variables)]
                let messages: Vec<Message> = packets
                    .into_iter()
                    .filter_map(|publish: Publish| -> Option<Message> {
                        match publish.try_into() {
                            Ok(msg) => Some(msg),
                            Err(e) => {
                                log::warn!("Failed to parse message: {e}");
                                None
                            }
                        }
                    })
                    .collect();
                if let Err(e) = message_sender.send(messages).await {
                    log::warn!("Failed to send message: {e}");
                }
                sleep(Duration::from_secs(2));
            }
        });
        (handle, sender)
    }
}
