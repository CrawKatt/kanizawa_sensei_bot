use dotenv::dotenv;
use crate::prelude::*;

mod utils;
mod error;
mod prelude;
mod enums;
mod handlers;
mod commands;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");
    dotenv().ok();

    run().await;
}

