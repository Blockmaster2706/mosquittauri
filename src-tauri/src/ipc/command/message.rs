use itertools::Itertools;
use tauri::AppHandle;

use crate::{
    ipc::event::{MqttError, MqttPullEvent},
    model::Message,
};

#[tauri::command]
pub async fn get_messages(app: AppHandle) -> tauri::Result<()> {
    let messages = match Message::find_by_enabled_server().await {
        Ok(messages) => messages,
        Err(e) => {
            log::error!("Failed to log ");
            MqttError::new(&e).send(&app);
            Err(e)?
        }
    };
    for batch in &messages.into_iter().chunks(100) {
        MqttPullEvent::new(batch.collect()).send(&app)?;
    }

    Ok(())
}
