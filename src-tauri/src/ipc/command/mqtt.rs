use std::{sync::atomic::Ordering, time::Duration};

use anyhow::{Context, Result};
use tauri::{AppHandle, Listener};

use crate::{
    ipc::event::{
        MqttConnectEvent, MqttDisconnectEvent, MqttDisconnectRequest, MqttError, MqttPullEvent,
        MqttSendEvent, MsqtEvent, TopicStateEvent,
    },
    model::{MsqtDao, Server, Session, Topic},
    mqtt::MqttPool,
};

#[tauri::command]
pub async fn mqtt_connect(app: AppHandle) -> tauri::Result<()> {
    if let Err(e) = mqtt_connect_internal(&app) {
        MqttError::new(&e).send(&app);
        log::error!("Mqtt connect failed: {e}")
    }
    Ok(())
}

fn mqtt_connect_internal(app: &AppHandle) -> Result<()> {
    let server_id = Session::get_or_init()
        .context("Failed to get session")?
        .server_id()
        .context("failed to get selected server id")?;
    let server = Server::find_by_id(server_id).context("Failed to get selected server")?;
    let mut pool = MqttPool::new(server.get_mqtt_options());
    let running = pool.get_running_atomic();
    let running_disconnect = running.clone();
    app.listen(MqttDisconnectRequest::ID, move |_event| {
        running_disconnect.store(false, Ordering::Relaxed);
    });
    let msg_sender = pool.get_msg_sender();
    app.listen(MqttSendEvent::ID, move |event| {
        let send_event = match serde_json::from_str::<MqttSendEvent>(event.payload()) {
            Ok(event) => event,
            Err(e) => {
                log::warn!("Failed to parse send event: {e}");
                return;
            }
        };
        if let Err(e) = msg_sender.send(send_event) {
            log::error!("Failed to send message: {e}")
        }
    });

    // Subscribe to all topics for selected server
    let topic_sender = pool.get_topic_sender();
    if let Err(e) =
        topic_sender.send(Topic::find_enabled_by_selected_server()?.context("No server selected")?)
    {
        log::error!("Failed to subscribe to topics {e}")
    }

    app.listen(TopicStateEvent::ID, move |event| {
        let state_event = match serde_json::from_str::<TopicStateEvent>(event.payload()) {
            Ok(event) => event,
            Err(e) => {
                log::warn!("Failed to parse send event: {e}");
                return;
            }
        };
        let topic = match Topic::find_by_id(state_event.id()) {
            Ok(topic) => topic,
            Err(e) => {
                log::error!("Failed to get changed topic:  {e}");
                return;
            }
        };
        if let Err(e) = topic_sender.send(vec![topic]) {
            log::error!("Faled to send topic state to mqtt pool {e}")
        }
    });

    MqttConnectEvent::new().send(app)?;
    let msg_receiver = pool
        .get_msg_receiver()
        .context("msg receiver already taken")?;
    while running.load(Ordering::Relaxed) {
        let Ok(batch) = msg_receiver.recv_timeout(Duration::from_millis(1000)) else {
            continue;
        };
        if let Err(e) = MqttPullEvent::new(batch).send(app) {
            log::error!("Failed to send mqtt sync event: {e}");
            continue;
        }
    }
    pool.disconnect();
    MqttDisconnectEvent::new().send(app)?;
    Ok(())
}
