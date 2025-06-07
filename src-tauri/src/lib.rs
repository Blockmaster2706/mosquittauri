#![warn(clippy::all)]
#![allow(unexpected_cfgs)]

use std::{
    sync::mpsc::{channel, Sender},
    thread::spawn,
};

use anyhow::{Context, Result};
use chrono::Local;

use ipc::{
    command,
    event::{LogEvent, MsqtEvent},
};
use log::LevelFilter;
use tauri::AppHandle;
use tauri_plugin_log::{
    fern::{log_file, Dispatch, Output},
    Target, TargetKind,
};

mod conf;
mod ipc;
mod model;
mod mqtt;
#[cfg(test)]
mod test;
mod utils;

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1"); // Disable dmabuf renderer for WebKitGTK so that the program starts correctly on all Linux Distros including Fedora
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::get_servers,
            command::add_server,
            command::edit_server,
            command::delete_server,
            command::select_server,
            command::get_topics,
            command::add_topic,
            command::edit_topic,
            command::delete_topic,
            command::set_topic_enabled,
            command::is_listen_all_topics,
            command::set_listen_all_topics,
            command::mqtt_connect,
        ])
        .setup(|app| {
            let sender = start_log_event_listener(app.handle().clone());
            let handle = app.handle();
            handle.plugin(
                tauri_plugin_log::Builder::default()
                    .level(LOG_LEVEL)
                    .format(|out, msg, record| {
                        let now = Local::now();
                        out.finish(format_args!(
                            "{}|{}|{}|{} ::: {}",
                            now.format("%Y.%m.%d"),
                            now.format("%H:%M:%S"),
                            record.module_path().unwrap_or("???"),
                            record.level(),
                            msg
                        ))
                    })
                    .target(Target::new(TargetKind::Dispatch(
                        Dispatch::new()
                            .chain(log_file("msqt.log")?)
                            .chain(Output::call(move |record| {
                                let event = match LogEvent::try_from_record(record) {
                                    Ok(event) => event,
                                    Err(e) => {
                                        eprintln!("Failed to generate LogEvent from record: {e}");
                                        return;
                                    }
                                };
                                sender
                                    .send(event)
                                    .expect("Failed to send log event to channel");
                            })),
                    )))
                    .build(),
            )?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .context("error while running tauri application")?;
    Ok(())
}

fn start_log_event_listener(app: AppHandle) -> Sender<LogEvent> {
    let (log_sender, log_receiver) = channel::<LogEvent>();
    spawn(move || loop {
        match log_receiver.recv() {
            Ok(event) => {
                if let Err(e) = event.send(&app) {
                    eprintln!("Failed to send log event: {e}");
                    continue;
                }
            }
            Err(e) => {
                eprintln!("Failed to receive log event: {e}")
            }
        }
    });
    log_sender
}
