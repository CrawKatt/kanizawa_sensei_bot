use std::thread::sleep;
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
use crate::utils::load_data;

#[tokio::main]
async fn main() {
    DB.connect::<Mem>(()).await.unwrap_or_else(|why| panic!("Ocurrio un error al conectar a la base de datos {why:#?}"));
    load_data().await.unwrap_or_else(|why| eprintln!("Ocurrio un error al cargar los datos: {why:#?}"));
    sleep(std::time::Duration::from_secs(3));
    println!("Base de datos cargada correctamente");
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");
    dotenv().ok();

    run().await;
}
