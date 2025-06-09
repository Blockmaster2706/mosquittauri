use chrono::Local;
use log::LevelFilter;
use tauri::async_runtime::block_on;
use tauri_plugin_log::fern::{log_file, Dispatch};

use crate::model::Session;

mod models;
mod mqtt;

pub fn init() {
    Dispatch::new()
        .level(LevelFilter::Trace)
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
        .chain(std::io::stderr())
        .chain(log_file("msqt_log_test.log").expect("Failed to init test log file"))
        .apply()
        .expect("Failed to init logger");
    block_on(Session::get_or_init()).expect("Failed to init session");
}
