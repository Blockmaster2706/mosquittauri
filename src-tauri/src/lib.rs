#![warn(clippy::all)]
#![allow(unexpected_cfgs)]

use std::{
    fs,
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
use model::Session;
use tauri::{async_runtime::block_on, AppHandle};
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
    log_rotation(); // Rotate logs to keep only the last 3 log files

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
            command::get_messages,
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
                            now.format("%Y-%m-%d"),
                            now.format("%H:%M:%S"),
                            record.module_path().unwrap_or("???"),
                            record.level(),
                            msg
                        ))
                    })
                    .target(Target::new(TargetKind::Dispatch(
                        Dispatch::new()
                            .chain(log_file(format!(
                                "msqt_log_{}.log",
                                chrono::Local::now().format("%d_%m_%Y_%H%M")
                            ))?)
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
            block_on(Session::get_or_init()).context("Failed to init session")?;
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

fn log_rotation() {
    let mut log_files: Vec<_> = fs::read_dir(std::env::current_dir().unwrap())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file() && entry.file_name().to_string_lossy().contains("msqt_log_")
        })
        .collect();

    if log_files.len() >= 3 {
        log_files.sort_by_key(|entry| entry.metadata().and_then(|m| m.modified()).ok());

        if let Some(oldest_file) = log_files.first() {
            fs::remove_file(oldest_file.path()).unwrap();
            log::debug!("{}{:?}", "Deleted oldest log file: ", oldest_file.path());
        }
    }
}
