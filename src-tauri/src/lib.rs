#![warn(clippy::all)]
#![allow(unexpected_cfgs)]

use anyhow::{Context, Result};
use chrono::Local;

use ipc::command;

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
            command::add_server,
            command::edit_server,
            command::delete_server,
            command::select_server
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
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
