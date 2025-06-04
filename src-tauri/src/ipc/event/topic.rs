use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::model::Topic;

// include!("../../../gen/proto/event.topic.v1.rs");

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicUpdate {
    list: Option<Vec<Topic>>,
}
impl TopicUpdate {
    const ID: &str = "topic-update";
    pub fn send(app: &AppHandle) -> tauri::Result<()> {
        let topics = match Topic::find_by_selected_server() {
            Ok(topics) => topics,
            Err(e) => {
                log::error!("Failed to get all topics {e}");
                TopicError::send(app, &e);
                return Err(e.into());
            }
        };
        app.emit(Self::ID, TopicUpdate { list: topics })
            .inspect_err(|e| log::error!("Failed to send Topic Update event {e}"))?;
        Ok(())
    }
}

// #[derive(Clone, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TopicSelected {
//     id: u64,
// }
// impl TopicSelected {
//     const ID: &str = "topic-selected";
//     pub fn send(app: &AppHandle, id: u64) {
//         if let Err(e) = app.emit(Self::ID, TopicSelected { id }) {
//             log::error!("Failed to send Topic Error Event: {e:?}");
//         }
//     }
// }

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicError {
    msg: String,
}
impl TopicError {
    const ID: &str = "topic-error";
    pub fn send(app: &AppHandle, msg: &impl ToString) {
        if let Err(e) = app.emit(
            Self::ID,
            TopicError {
                msg: msg.to_string(),
            },
        ) {
            log::error!("Failed to send Topic Error Event: {e:?}");
        }
    }
}
