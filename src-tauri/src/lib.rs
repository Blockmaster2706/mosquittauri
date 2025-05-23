use anyhow::{Context, Result};
use chrono::Local;
use model::{MsqtDao, Server};

mod commands;
mod conf;
mod events;
mod model;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1"); // Disable dmabuf renderer for WebKitGTK so that the program starts correctly on all Linux Distros including Fedora
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .format(|out, msg, record| {
                            let now = Local::now();
                            out.finish(format_args!(
                                "{}|{}|{}|{}|{}",
                                now.format("%Y.%M.%D"),
                                now.format("%H:%M:%S"),
                                record.module_path().unwrap_or("???"),
                                record.level(),
                                msg
                            ))
                        })
                        .build(),
                )?;
            }
            test_json_storage();
            Ok(())
        })
        .run(tauri::generate_context!())
        .context("error while running tauri application")?;
    Ok(())
}

fn test_json_storage() {
    fn print_servers() {
        println!("{:?}", Server::find_all());
    }

    print_servers();
    Server::try_new("example.com", "client")
        .err()
        .inspect(|e| log::error!("Failed to add server {e:#?}"));
    print_servers();

    log::info!("Test sucessful")
}
