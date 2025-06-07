#![cfg(test)]
use std::{
    thread::{self, sleep},
    time::Duration,
};

use anyhow::Result;
use ntest::timeout;
use tauri::async_runtime::block_on;
use test_context::{test_context, TestContext};

use crate::{
    ipc::event::MqttSendEvent,
    model::{Message, Server},
    mqtt::MqttPool,
};

struct TestMosquitto {
    server: Server,
}

impl TestContext for TestMosquitto {
    fn setup() -> Self {
        let server = match Server::find_by_name("test_mosquitto") {
            Ok(server) => server,
            Err(_) => Server::try_new(
                "test_mosquitto",
                "test.mosquitto.org",
                1883_u16,
                "mosquitto_test",
            )
            .expect("Failed to create test server"),
        };
        Self { server }
    }
}

#[test_context(TestMosquitto)]
// This test may soft lock on fail
#[timeout(60_000)]
#[test]
fn test_mosquitto_msqttest(context: &mut TestMosquitto) -> Result<()> {
    super::init_loger();
    log::info!("create pool");
    let (mut pool, msg_sender, msg_receiver) = MqttPool::new(context.server.get_mqtt_options());
    log::info!("listen for messages to print");
    let message_out_handle = thread::spawn(move || match msg_receiver.recv() {
        Ok(messages) => {
            let payloads: Vec<&str> = messages.iter().map(Message::payload).collect();
            println!("Mqtt Messages: {payloads:#?}")
        }
        Err(e) => {
            log::warn!("Failed to recieve message to send: {e}");
        }
    });
    log::info!("subscibe to msqt_test");
    pool.add_subscriber("msqt_test")?;

    log::info!("send test message");
    msg_sender.send(MqttSendEvent::new("msqt_test", "Hallo"))?;

    log::info!("wait");
    sleep(Duration::from_secs(30));
    log::info!("disconnect");
    pool.disconnect();
    log::info!("wait for disconnect");
    block_on(async move {
        message_out_handle
            .join()
            .expect("Failed to wait for message thread");
    });
    Ok(())
}
