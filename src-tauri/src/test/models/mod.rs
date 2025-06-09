use std::{thread::sleep, time::Duration};

use anyhow::{Context, Result};
use tauri::async_runtime::block_on;

use crate::model::{MsqtDao, MsqtDto, Server};

fn print_servers() {
    println!("{:?}", block_on(Server::find_all()));
}

#[test]
fn test_database() -> Result<()> {
    print_servers();
    let server = block_on(Server::try_new(
        "example",
        "example.com",
        1883_u16,
        "client",
    ))
    .context("Failed to add server")?;
    print_servers();
    sleep(Duration::from_secs(3));
    Server::delete(server.id())?;
    Ok(())
}
