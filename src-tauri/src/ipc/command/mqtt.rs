use std::sync::atomic::Ordering;

use tauri::{
    async_runtime::channel,
    plugin::{Builder as PluginBuilder, TauriPlugin},
    AppHandle, Listener, Runtime,
};

use crate::{
    ipc::event::MqttDisconnectEvent,
    model::{MsqtDao, Server, Session},
    mqtt::MqttPool,
};

#[tauri::command]
async fn connect<R: Runtime>(app: AppHandle<R>) {
    let server_id = Session::get_or_init().unwrap().server_id().unwrap();
    let server = Server::find_by_id(server_id).unwrap();
    let (message_sender, mut message_receiver) = channel(32);
    let pool = MqttPool::new(server.get_mqtt_options(), message_sender);
    let running = pool.get_running_atomic();
    let running_disconnect = pool.get_running_atomic();
    app.listen(MqttDisconnectEvent::ID, move |_event| {
        running_disconnect.store(false, Ordering::Relaxed);
    });
    // let message_reveiver_handle = spawn(async move {
    while running.load(Ordering::Relaxed) {
        let mut messages = Vec::new();
        message_receiver
            .recv_many(&mut messages, MqttPool::MSG_BATCH_QUEUE_LEN)
            .await;
    }
    pool.disconnect();
    // });
}

pub fn mqtt_plugin<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("msqt_mqtt")
        .invoke_handler(tauri::generate_handler![connect])
        .build()
}
