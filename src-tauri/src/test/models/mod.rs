use anyhow::{Context, Result};
use tauri::async_runtime::block_on;

use crate::model::{MsqtDao, MsqtDto, Server};

#[test]
fn test_storage() -> Result<()> {
    // crate::test::init();

    // Create server
    let server = block_on(Server::try_new(
        "example",
        "example.com",
        1883_u16,
        "client",
    ))
    .context("Failed to add server")?;
    assert_eq!(
        block_on(Server::find_by_name("example"))
            .ok()
            .map(|server| server.id()),
        Some(server.id())
    );

    assert!(block_on(Server::find_all())?.contains(&server));

    // Cleanup
    block_on(Server::delete(server.id()))?;
    Ok(())
}
