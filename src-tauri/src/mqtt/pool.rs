use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, RecvTimeoutError, Sender},
        Arc,
    },
    thread::{spawn, JoinHandle},
    time::Duration,
};

use rumqttc::{AsyncClient, MqttOptions, SubscribeFilter};
use tauri::async_runtime as tk;

use super::lock::Lock;
use crate::{
    ipc::event::MqttSendEvent,
    model::{Message, Topic},
};

#[allow(dead_code)]
pub struct MqttPool {
    client: Arc<Lock<AsyncClient>>,
    running: Arc<AtomicBool>,
    topic_sender: Sender<Vec<Topic>>,
    msg_sender_sender: Sender<MqttSendEvent>,
    msg_listener_receiver: Option<Receiver<Vec<Message>>>,
    msg_listener_handle: JoinHandle<()>,
    topic_listener_handle: JoinHandle<()>,
    msg_sender_handle: JoinHandle<()>,
    server_listener_handle: JoinHandle<()>,
    capacity: usize,
}

impl MqttPool {
    pub const MQTT_CLIENT_CAP: usize = 16;

    #[must_use = "must use to get channels required to interact with this pool"]
    pub fn new(options: MqttOptions) -> Self {
        // create client
        let (client_raw, event_loop) = AsyncClient::new(options, Self::MQTT_CLIENT_CAP);
        let client = Arc::new(Lock::new(client_raw));
        let running = Arc::new(AtomicBool::new(true));

        // create channels
        let (msg_batch_sender, msg_listener_receiver) = channel();
        let (publish_sender, publish_receiver) = channel();
        let (msg_sender_sender, msg_sender_receiver) = channel();
        let (topic_sender, topic_receiver) = channel();

        // start internal listener and senders
        let msg_listener_handle = Self::start_packet_listener(
            running.clone(),
            publish_receiver,
            msg_batch_sender.clone(),
        );
        let topic_listener_handle =
            Self::start_topic_listener(client.clone(), running.clone(), topic_receiver);
        let msg_sender_handle = Self::start_msg_sender(
            client.clone(),
            running.clone(),
            msg_sender_receiver,
            msg_batch_sender,
        );
        let server_listener_handle =
            Self::start_server_listener(publish_sender, event_loop, running.clone());

        Self {
            client,
            msg_listener_handle,
            topic_listener_handle,
            msg_sender_handle,
            server_listener_handle,
            running: running.clone(),
            capacity: 10,
            topic_sender,
            msg_sender_sender,
            msg_listener_receiver: Some(msg_listener_receiver),
        }
    }
    fn start_topic_listener(
        client: Arc<Lock<AsyncClient>>,
        running: Arc<AtomicBool>,
        topic_receiver: Receiver<Vec<Topic>>,
    ) -> JoinHandle<()> {
        spawn(move || {
            while running.load(Ordering::Relaxed) {
                let topics: Vec<Topic> =
                    match topic_receiver.recv_timeout(Duration::from_millis(1500)) {
                        Ok(topics) => topics,
                        Err(RecvTimeoutError::Timeout) => continue,
                        Err(RecvTimeoutError::Disconnected) => break,
                    };

                log::trace!("{topics:?}");

                let subscribe: Vec<SubscribeFilter> = topics
                    .iter()
                    .filter(|t| t.is_enabled())
                    .map(|topic| topic.get_subscribe_filter())
                    .collect();
                log::debug!("subscribing to {subscribe:?}");

                let unsubscribe: Vec<&str> = topics
                    .iter()
                    .filter(|topic| !topic.is_enabled())
                    .map(|topic| topic.name())
                    .collect();
                log::debug!("unsubscribing from {unsubscribe:?}");

                if !subscribe.is_empty() {
                    match client.with(move |client| tk::block_on(client.subscribe_many(subscribe)))
                    {
                        Err(e) => log::error!("Failed to get lock to subscribe: {e}"),
                        Ok(Err(e)) => log::error!("Failed to subscribe to topic: {e}"),
                        Ok(Ok(())) => (),
                    }
                }
                if !unsubscribe.is_empty() {
                    for unsubscribe_topic in unsubscribe {
                        match client
                            .with(move |client| tk::block_on(client.unsubscribe(unsubscribe_topic)))
                        {
                            Err(e) => log::error!("Failed to get lock to subscribe: {e}"),
                            Ok(Err(e)) => log::error!("Failed to subscribe to topic: {e}"),
                            Ok(Ok(())) => (),
                        }
                    }
                }
            }
            log::debug!("stopped topic listener")
        })
    }
    pub fn get_running_atomic(&self) -> Arc<AtomicBool> {
        self.running.clone()
    }
    pub fn get_topic_sender(&mut self) -> Sender<Vec<Topic>> {
        self.topic_sender.clone()
    }
    pub fn get_msg_sender(&self) -> Sender<MqttSendEvent> {
        self.msg_sender_sender.clone()
    }
    pub fn get_msg_receiver(&mut self) -> Option<Receiver<Vec<Message>>> {
        self.msg_listener_receiver.take()
    }

    pub fn disconnect(self) {
        self.running.store(false, Ordering::Relaxed);
        if let Err(e) = self.msg_listener_handle.join() {
            log::warn!("Failed to stop message listener: {e:?}")
        };
        if let Err(e) = self.topic_listener_handle.join() {
            log::warn!("Failed to stop topic listener: {e:?}")
        };
    }
}
