use crate::model::{MsqtDao, Server};

fn print_servers() {
    println!("{:?}", Server::find_all());
}

#[test]
fn test_json_storage() {
    print_servers();
    Server::try_new("example", "example.com", "client")
        .err()
        .inspect(|e| log::error!("Failed to add server {e:#?}"));
    print_servers();
}
