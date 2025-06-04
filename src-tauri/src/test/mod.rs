use chrono::Local;
use log::LevelFilter;
use tauri_plugin_log::fern::Dispatch;

mod models;
mod mqtt;

pub fn init_loger() {
    Dispatch::new()
        .level(LevelFilter::Debug)
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
        .chain(std::io::stderr())
        .apply()
        .expect("Failed to init logger");
}
