extern crate dotenv;
// extern crate postgres;
extern crate envfile;
extern crate mysql;

extern crate chrono;

pub mod user {
    tonic::include_proto!("user");
}

use tonic::transport::Server;

use user::crud_server::CrudServer;

extern crate uuid;

extern crate console;
use console::Style;

mod db_connection;
mod env;
mod service;

use crate::service::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user = User::default();

    let blue = Style::new().blue();

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    Server::builder()
        .add_service(CrudServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
