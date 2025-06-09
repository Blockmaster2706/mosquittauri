#![cfg(test)]
use std::{
    sync::{atomic::Ordering, mpsc::RecvTimeoutError},
    thread::{sleep, spawn},
    time::Duration,
};

use anyhow::Result;
use ntest::timeout;
use tauri::async_runtime::block_on;
use test_context::{test_context, TestContext};

use crate::{
    ipc::event::MqttSendEvent,
    model::{Message, MsqtDao, MsqtDto, Server, Session, Topic},
    mqtt::MqttPool,
};

struct MqttTest {
    server: Server,
    topic: Topic,
}

impl TestContext for MqttTest {
    fn setup() -> Self {
        super::init();
        let broker_url =
            std::env::var("MSQT_TEST_BROKER_URL").unwrap_or(String::from("test.mosquitto.org"));
        log::debug!("testing with broker {}", broker_url);
        let server = match block_on(Server::find_by_name("msqt_test")) {
            Ok(server) => server,
            Err(_) => block_on(Server::try_new(
                "msqt_test",
                broker_url,
                1883_u16,
                "mosquitto_test",
            ))
            .expect("Failed to create test server"),
        };
        block_on(Session::select_server(server.id())).expect("Failed to select error");
        let topic = block_on(Topic::try_new(server.id(), "msqt_test"))
            .expect("Failed to create test topic");
        block_on(Topic::set_enabled(topic.id(), true)).expect("Failed to enable test topic");
        let topic = block_on(topic.refresh()).unwrap();
        Self { server, topic }
    }
    fn teardown(self) {
        block_on(async {
            Server::delete(self.server.id())
                .await
                .expect("Failed to delte mqtt test server");
            Topic::delete(self.topic.id())
                .await
                .expect("Failed to delete test topic")
        })
    }
}

#[test_context(MqttTest)]
// This test may soft lock on fail
#[timeout(60_000)]
#[test]
fn mqtt(context: &mut MqttTest) -> Result<()> {
    log::info!("create pool");
    let mut pool = MqttPool::new(context.server.get_mqtt_options());
    let running = pool.get_running_atomic();
    log::info!("listen for messages to print");

    let msg_receiver = pool.get_msg_receiver().expect("receiver already used");

    let message_out_handle = spawn(move || {
        while running.load(Ordering::Relaxed) {
            match msg_receiver.recv_timeout(Duration::from_millis(1500)) {
                Ok(messages) => {
                    let payloads: Vec<&str> = messages.iter().map(Message::payload).collect();
                    println!("Mqtt Messages: {payloads:#?}");
                    log::info!("Mqtt Messages: {payloads:#?}");
                }
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
        log::debug!("Stopped test message batch listener");
    });
    log::info!("subscibe to msqt_test");
    pool.get_topic_sender().send(vec![context.topic.clone()])?;

    log::info!("wait for listeners to start");
    sleep(Duration::from_secs(5));

    log::info!("send test message");
    pool.get_msg_sender()
        .send(MqttSendEvent::new(context.topic.name(), "Hallo"))?;

    log::info!("wait for message parsed");
    sleep(Duration::from_secs(25));
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
