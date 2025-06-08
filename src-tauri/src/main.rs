// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all)]

use anyhow::Result;

fn main() -> Result<()> {
    app_lib::run()?;
    Ok(())
}
