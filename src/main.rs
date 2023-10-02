use dotenv::dotenv;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;
use teloxide::prelude::Dispatcher;
use teloxide_core::prelude::RequesterExt;
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::{ParseMode, Update};

mod utils;
mod error;
mod prelude;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Iniciando Bot...");

    dotenv().ok();

    run().await
}

async fn run() {

    let bot = teloxide::Bot::from_env().parse_mode(ParseMode::Html);

    let handler = dptree::entry()
        .inspect(|u: Update| {
            log::info!("Update: {:?}", u);
        })
        .branch(
            Update::filter_message()
                .endpoint(message)
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn message() -> ResponseResult<()> {
    todo!()
}