use tauri::AppHandle;

use crate::ipc::event::{MsqtEvent, TopicError, TopicUpdate};
use crate::model::{Session, Topic};

#[tauri::command]
pub async fn add_topic(server_id: u64, name: String, app: AppHandle) -> tauri::Result<()> {
    if let Err(e) = Topic::try_new(server_id, name) {
        log::error!("Failed to create topic: {e}");
        TopicError::new(&e).send(&app)?;
    }
    TopicUpdate::from_all(&app)?.send(&app)?;
    Ok(())
}

#[tauri::command]
pub async fn set_topic_enabled(id: u64, enabled: bool, app: AppHandle) -> tauri::Result<()> {
    if let Err(e) = Topic::set_enabled(id, enabled) {
        log::error!("Failed to create topic: {e}");
        TopicError::new(&e).send(&app)?;
    }
    TopicUpdate::from_all(&app)?.send(&app)
}

#[tauri::command]
pub async fn edit_topic(id: u64, name: String, app: AppHandle) -> tauri::Result<()> {
    Topic::update(id, name).inspect_err(|e| log::error!("Failed to create topic: {e}"))?;
    TopicUpdate::from_all(&app)?.send(&app)
}

#[tauri::command]
pub async fn delete_topic(id: u64, app: AppHandle) -> tauri::Result<()> {
    if let Err(e) = Topic::delete(id) {
        log::error!("Failed to create topic: {e}");
        TopicError::new(&e).send(&app)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_topics(app: AppHandle) -> tauri::Result<()> {
    TopicUpdate::from_all(&app)?.send(&app)
}

#[tauri::command]
pub async fn is_listen_all_topics() -> tauri::Result<bool> {
    Ok(Session::get_or_init()?.listen_all_topics())
}

#[tauri::command]
pub async fn set_listen_all_topics(app: AppHandle, enabled: bool) -> tauri::Result<()> {
    if let Err(e) = Session::set_listen_all_topics(enabled) {
        TopicError::new(&e).send(&app)?;
    }
    TopicUpdate::from_all(&app)?.send(&app)?;
    Ok(())
}
