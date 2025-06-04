#![warn(clippy::all)]
#![allow(unexpected_cfgs)]

use anyhow::{Context, Result};
use chrono::Local;

use ipc::command;
use tauri_plugin_log::{
    fern::{log_file, Dispatch},
    Target, TargetKind,
};

mod conf;
mod ipc;
mod model;
mod mqtt;
#[cfg(test)]
mod test;
mod utils;

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
            command::listen_all_topics,
            command::set_listen_all_topics,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                let handle = app.handle();
                handle.plugin(command::mqtt_plugin())?;
                handle.plugin(
                    tauri_plugin_log::Builder::default()
                        .target(Target::new(TargetKind::Dispatch(
                            Dispatch::new().chain(log_file("msqt.log")?),
                        )))
                        .level(log::LevelFilter::Info)
                        .format(|out, msg, record| {
                            let now = Local::now();
                            out.finish(format_args!(
                                "{}|{}|{}|{}|{}",
                                now.format("%Y.%m.%d"),
                                now.format("%H:%M:%S"),
                                record.module_path().unwrap_or("???"),
                                record.level(),
                                msg
                            ))
                        })
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .context("error while running tauri application")?;
    Ok(())
}
