use rumqttc::{ConnectionError, Event, EventLoop, Packet, Publish};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, Sender},
        Arc,
    },
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};
use tauri::async_runtime as tk;

use crate::model::Message;

use super::MqttPool;

impl MqttPool {
    pub(super) fn start_server_listener(
        publish_sender: Sender<Publish>,
        mut eventloop: EventLoop,
        running: Arc<AtomicBool>,
    ) -> JoinHandle<()> {
        spawn(move || {
            let mut instant = Instant::now();
            loop {
                let event = tk::block_on(eventloop.poll());

                // parse event
                match event {
                    Ok(Event::Incoming(Packet::Publish(publish))) => {
                        log::trace!("incoming publish packet");
                        if let Err(e) = publish_sender.send(publish) {
                            log::warn!("Failed to send publish packet to channel: {e}");
                        }
                    }
                    Ok(event) => log::trace!("ignored packet {event:?} "),
                    Err(e) => {
                        log::error!("Failed to receive event from server: {e}");
                        running.store(false, Ordering::Relaxed);
                        break;
                    }
                }

                sleep(Duration::from_millis(500));
                // check if disconnect event sent
                if instant.elapsed().as_millis() > 1500 {
                    log::trace!("Server listener: Check if running");
                    if !running.load(Ordering::Relaxed) {
                        break;
                    }
                    instant = Instant::now();
                }
            }
            log::debug!("stopped mqtt connection listener");
        })
    }

    pub(super) fn start_packet_listener(
        running: Arc<AtomicBool>,
        publish_receiver: Receiver<Publish>,
        message_sender: Sender<Vec<Message>>,
    ) -> JoinHandle<()> {
        let handle = spawn(move || {
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();
                let mut messages = Vec::new();
                while start.elapsed().as_millis() < 1500 {
                    let Ok(publish) = publish_receiver.recv_timeout(Duration::from_millis(1000))
                    else {
                        continue;
                    };
                    log::trace!("parsing publish packet");
                    let msg = match publish.try_into() {
                        Ok(msg) => msg,
                        Err(e) => {
                            log::warn!("Failed to parse message: {e}");
                            continue;
                        }
                    };
                    messages.push(msg);
                }
                if messages.is_empty() {
                    continue;
                }
                log::trace!("sending message batch");
                if let Err(e) = message_sender.send(messages) {
                    log::warn!("Failed to send message to send channel: {e}");
                }
            }
            log::debug!("Stopped package listener")
        });
        handle
    }
}
