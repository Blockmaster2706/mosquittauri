use std::sync::atomic::Ordering;
use std::sync::mpsc::{RecvTimeoutError, Sender};
use std::sync::Arc;
use std::sync::{atomic::AtomicBool, mpsc::Receiver};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

use rumqttc::{AsyncClient, QoS};
use tauri::async_runtime as tk;

use crate::ipc::event::MqttSendEvent;
use crate::model::Message;

use super::{lock::Lock, MqttPool};

impl MqttPool {
    pub(super) fn start_msg_sender(
        client: Arc<Lock<AsyncClient>>,
        running: Arc<AtomicBool>,
        send_event_receiver: Receiver<MqttSendEvent>,
        message_batch_sender: Sender<Vec<Message>>,
    ) -> JoinHandle<()> {
        spawn(move || {
            while running.load(Ordering::Relaxed) {
                match send_event_receiver.recv_timeout(Duration::from_millis(1500)) {
                    Ok(msg) => {
                        log::debug!("incoming send event");
                        if let Err(e) = client.with(|client| {
                            tk::block_on(client.publish(
                                msg.topic(),
                                QoS::ExactlyOnce,
                                false,
                                msg.payload(),
                            ))
                        }) {
                            log::warn!("Failed to publish mqtt message: {e:?}");
                            continue;
                        }
                        // match Message::try_from(msg) {
                        //     Ok(msg) => {
                        //         if let Err(e) = message_batch_sender.send(vec![msg]) {
                        //             log::warn!(
                        //                 "Failed to send message generated from send event: {e}"
                        //             )
                        //         }
                        //         log::trace!("send copy of sent message to batch listener");
                        //     }
                        //     Err(e) => log::warn!("Failed generate message from send event: {e}"),
                        // }
                    }
                    Err(RecvTimeoutError::Timeout) => continue,
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
            log::debug!("Stopped message sender")
        })
    }
}
