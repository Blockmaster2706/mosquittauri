#![cfg(test)]
use std::{thread::sleep, time::Duration};

use anyhow::Result;
use ntest::timeout;
use tauri::async_runtime::{block_on, channel, spawn};
use test_context::{test_context, TestContext};

use crate::{
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
#[timeout(30_000)]
#[test]
fn test_mosquitto_msqttest(context: &mut TestMosquitto) -> Result<()> {
    super::init_loger();
    log::info!("create channel");
    let (message_sender, mut message_receiver) = channel::<Vec<Message>>(32);
    log::info!("listen for messages to print");
    let message_out_handle = spawn(async move {
        while let Some(msgs) = message_receiver.recv().await {
            let payloads: Vec<&str> = msgs.iter().map(|msg| msg.payload()).collect();
            println!("Mqtt Messages: {payloads:#?}")
        }
    });
    log::info!("create pool");
    let mut pool = MqttPool::new(context.server.get_mqtt_options(), message_sender);
    log::info!("subscibe to msqt_test");
    pool.add_subscriber("msqt_test")?;
    log::info!("wait 10 sec");
    sleep(Duration::from_secs(10));
    log::info!("disconnect");
    pool.disconnect();
    log::info!("wait for disconnect");
    block_on(async move {
        message_out_handle
            .await
            .expect("Failed to parse message handle");
    });
    Ok(())
}
