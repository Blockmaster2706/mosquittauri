use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::model::Topic;

use super::MsqtEvent;

// include!("../../../gen/proto/event.topic.v1.rs");

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicUpdate {
    list: Option<Vec<Topic>>,
}

impl MsqtEvent for TopicUpdate {
    const ID: &str = "topic-update";
}
impl TopicUpdate {
    pub fn from_all(app: &AppHandle) -> tauri::Result<Self> {
        let list = match Topic::find_by_selected_server() {
            Ok(list) => list,
            Err(e) => {
                log::error!("Failed to get all topics {e}");
                let _ = TopicError::new(&e).send(app);
                return Err(e.into());
            }
        };
        Ok(Self { list })
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

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicError {
    msg: String,
}

impl MsqtEvent for TopicError {
    const ID: &str = "topic-error";
}

impl TopicError {
    pub fn new(msg: &impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
