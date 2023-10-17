use crate::prelude::*;
use dotenv::dotenv;
use surrealdb::engine::local::Mem;

mod commands;
mod enums;
mod error;
mod handlers;
mod prelude;
mod utils;
use crate::utils::db::DB;

#[tokio::main]
async fn main() {
    DB.connect::<Mem>(()).await.expect("TODO: panic message");
    //println!("DB: {DB:?}");
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");
    dotenv().ok();

    run().await;
}
