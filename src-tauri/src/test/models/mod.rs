use anyhow::Result;

use crate::model::{MsqtDao, Server};

fn print_servers() {
    println!("{:?}", Server::find_all());
}

#[test]
fn test_json_storage() -> Result<()> {
    print_servers();
    Server::try_new("example", "example.com", 1883_u16, "client")
        .err()
        .inspect(|e| log::error!("Failed to add server {e:#?}"));
    print_servers();
    Ok(())
}
