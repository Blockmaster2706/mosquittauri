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
use tauri::async_runtime::{channel, spawn, JoinHandle, Sender};

use crate::model::Message;

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
    pub fn new(options: MqttOptions) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let (msg_listener_handle, publish_sender) = Self::start_msg_listener(running.clone());
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
            client,
            connection,
            self.publish_sender.clone(),
            self.running.clone(),
        );
        self.connections.push(mqtt_connection);
        Ok(())
    }
    pub fn disconnect(self) {
        self.running.store(false, Ordering::Relaxed);
        while self.msg_listener_handle.inner().is_finished() {}
    }
    pub fn start_msg_listener(running: Arc<AtomicBool>) -> (JoinHandle<()>, Sender<Publish>) {
        let (sender, mut receiver) = channel(96);
        let handle = spawn(async move {
            while running.load(Ordering::Relaxed) {
                let mut packets = Vec::new();
                receiver.blocking_recv_many(&mut packets, 96);
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
                // TDDO: Send Event to frontend
                sleep(Duration::from_secs(2));
            }
        });
        (handle, sender)
    }
}
