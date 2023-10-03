use crate::prelude::*;
use dotenv::dotenv;

mod commands;
mod enums;
mod error;
mod handlers;
mod prelude;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");
    dotenv().ok();

    run().await;
}
