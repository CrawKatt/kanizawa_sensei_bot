use teloxide::dispatching::UpdateFilterExt;
use teloxide_core::adaptors::DefaultParseMode;
use teloxide_core::types::{ParseMode, Update};
use crate::handlers::command::{for_database, handler};
use teloxide::dptree;
use teloxide::prelude::Dispatcher;
use teloxide_core::prelude::RequesterExt;

pub type Bot = DefaultParseMode<teloxide::Bot>;

pub async fn run() {

    let bot = teloxide::Bot::from_env().parse_mode(ParseMode::Html);

    let handler = dptree::entry()
        .inspect(|u: Update| {
            println!("Update: {u:#?}");
        })
        .branch(
            Update::filter_message()
                .endpoint(handler)
        )
        .branch(
            Update::filter_message()
                .endpoint(for_database)
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
