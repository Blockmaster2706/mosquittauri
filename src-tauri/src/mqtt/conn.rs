use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::{spawn, JoinHandle},
    time::{Duration, Instant},
};

use anyhow::Result;
use rumqttc::{ConnectionError, Event, EventLoop, Packet, Publish};
use tauri::async_runtime as tk;
use tokio::time::timeout;

pub struct MqttConnection {
    handle: JoinHandle<()>,
}

impl MqttConnection {
    pub fn new(
        eventloop: EventLoop,
        publish_sender: Sender<Publish>,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            handle: Self::start_listener(publish_sender, eventloop, running),
        }
    }

    pub fn await_disconnect(self) {
        if let Err(e) = self.handle.join() {
            log::warn!("Failed to stop thread of mqtt connettion: {e:#?}")
        }
    }

    fn start_listener(
        publish_sender: Sender<Publish>,
        mut eventloop: EventLoop,
        running: Arc<AtomicBool>,
    ) -> JoinHandle<()> {
        spawn(move || {
            let mut instant = Instant::now();
            loop {
                let Ok(res) = tk::block_on(tokio::time::timeout(
                    Duration::from_secs(6),
                    eventloop.poll(),
                )) else {
                    continue;
                };
                Self::parse_event(res, &publish_sender);
                if instant.elapsed().as_millis() > 1500 {
                    log::trace!("Connection: Check if running");
                    if !running.load(Ordering::Relaxed) {
                        break;
                    }
                    instant = Instant::now();
                }
            }
            log::info!("stopped mqtt connection listener");
        })
    }

    fn parse_event(event: Result<Event, ConnectionError>, publish_sender: &Sender<Publish>) {
        match event {
            Ok(rumqttc::Event::Incoming(Packet::Publish(publish))) => {
                log::trace!("incoming publish packet");
                if let Err(e) = publish_sender.send(publish) {
                    log::warn!("Failed to send publish packet to channel: {e}");
                }
            }
            Ok(event) => log::trace!("ignored incomong packet {event:?} "),
            Err(e) => log::warn!("Failed to parse event: {e}"),
        }
    }
}
