use std::{thread::sleep, time::Duration};

use anyhow::{Context, Result};

use crate::model::{MsqtDao, MsqtDto, Server};

fn print_servers() {
    println!("{:?}", Server::find_all());
}

#[test]
fn test_json_storage() -> Result<()> {
    print_servers();
    let server = Server::try_new("example", "example.com", 1883_u16, "client")
        .context("Failed to add server")?;
    print_servers();
    sleep(Duration::from_secs(3));
    Server::delete(server.id())?;
    Ok(())
}
