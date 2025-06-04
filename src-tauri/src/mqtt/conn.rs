use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};

use anyhow::Result;
use rumqttc::{Connection, ConnectionError, Event, Packet, Publish};
use tauri::async_runtime::{spawn, JoinHandle, Sender};

pub struct MqttConnection {
    handle: JoinHandle<()>,
}

impl MqttConnection {
    pub fn new(
        connection: Connection,
        publish_sender: Sender<Publish>,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            handle: Self::start_listener(publish_sender, connection, running),
        }
    }

    pub async fn await_disconnect(self) {
        while !self.handle.inner().is_finished() {}
    }

    fn start_listener(
        publish_sender: Sender<Publish>,
        mut connection: Connection,
        running: Arc<AtomicBool>,
    ) -> JoinHandle<()> {
        spawn(async move {
            // let mut check_cycle = 0;
            let mut instant = Instant::now();
            for res in connection.iter() {
                let parse_event = Self::parse_event(res, &publish_sender);
                // check_cycle += 1;
                if instant.elapsed().as_millis() > 1500 {
                    // log::debug!("Connection: Check if running");
                    if !running.load(Ordering::Relaxed) {
                        return;
                    }
                    instant = Instant::now();
                    // check_cycle = 0;
                }
                parse_event.await;
                // sleep(Duration::from_millis(1500));
            }
        })
    }

    async fn parse_event(event: Result<Event, ConnectionError>, publish_sender: &Sender<Publish>) {
        match event {
            Ok(rumqttc::Event::Incoming(Packet::Publish(publish))) => {
                publish_sender
                    .blocking_send(publish)
                    .err()
                    .inspect(|e| log::warn!("Failed to send publish packet to channel: {e}"));
            }
            Ok(_) => (),
            Err(e) => log::warn!("Failed to parse event: {e}"),
        }
    }
}
