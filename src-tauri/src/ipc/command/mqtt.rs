use std::{sync::atomic::Ordering, time::Duration};

use anyhow::Context;
use tauri::{AppHandle, Listener};

use crate::{
    ipc::event::{MqttDisconnectEvent, MqttSendEvent, MqttSyncEvent},
    model::{MsqtDao, Server, Session, Topic},
    mqtt::MqttPool,
};

#[tauri::command]
pub async fn mqtt_connect(app: AppHandle) -> tauri::Result<()> {
    let server_id = Session::get_or_init()
        .context("Failed to get session")?
        .server_id()
        .unwrap();
    let server = Server::find_by_id(server_id).unwrap();
    let (mut pool, msg_sender, msg_recever) = MqttPool::new(server.get_mqtt_options());
    let running = pool.get_running_atomic();
    let running_disconnect = running.clone();
    app.listen(MqttDisconnectEvent::ID, move |_event| {
        running_disconnect.store(false, Ordering::Relaxed);
    });
    app.listen(MqttSendEvent::ID, move |event| {
        let send_event = match serde_json::from_str::<MqttSendEvent>(event.payload()) {
            Ok(event) => event,
            Err(e) => {
                log::warn!("Failed to parse send event: {e}");
                return;
            }
        };
        let msg_sender = &msg_sender;
        if let Err(e) = msg_sender.send(send_event) {
            log::error!("Failed to send message: {e}")
        }
    });

    // Subscribe to all topics for selected server
    for topic in Topic::find_by_selected_server()?.context("No server selected")? {
        if let Err(e) = pool.add_subscriber(topic.name()) {
            log::error!("Failed to subscribe to topic {e}")
        }
    }

    while running.load(Ordering::Relaxed) {
        let Ok(batch) = msg_recever.recv_timeout(Duration::from_millis(1000)) else {
            continue;
        };
        if let Err(e) = MqttSyncEvent::new(batch).send(&app) {
            log::error!("Failed to send mqtt sync event: {e}");
            continue;
        }
    }
    pool.disconnect();
    Ok(())
}
