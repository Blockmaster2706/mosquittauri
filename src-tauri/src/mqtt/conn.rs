use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::Result;
use rumqttc::{Client, Connection, ConnectionError, Event, Packet, Publish};
use tauri::async_runtime::Sender;

pub struct MqttConnection {
    client: Client,
    connection: Connection,
    publish_sender: Sender<Publish>,
    running: Arc<AtomicBool>,
}

impl MqttConnection {
    pub fn new(
        client: Client,
        connection: Connection,
        publish_sender: Sender<Publish>,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            client,
            connection,
            publish_sender,
            running,
        }
    }

    pub fn listen(&mut self) -> Result<()> {
        let mut check_cycle = 0;
        while let Ok(event) = self.connection.recv_timeout(Duration::from_millis(1500))
        // .inspect_err(|err| log::warn!("Event: {err:?}"))
        {
            self.parse_event(event);
            check_cycle += 1;
            if check_cycle >= 3 {
                if !self.running.load(Ordering::Relaxed) {
                    return Ok(());
                }
                check_cycle = 0;
            }
        }
        Ok(())
    }

    fn parse_event(&self, event: Result<Event, ConnectionError>) {
        match event {
            Ok(rumqttc::Event::Incoming(Packet::Publish(publish))) => {
                self.publish_sender
                    .blocking_send(publish)
                    .err()
                    .inspect(|e| log::warn!("Failed to send publish packet to channel: {e}"));
            }
            Ok(_) => (),
            Err(e) => log::warn!("Failed to parse event: {e}"),
        }
    }
}
