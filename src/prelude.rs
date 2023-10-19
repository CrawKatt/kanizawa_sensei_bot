use crate::handlers::{
    chat_member_updates::{
        left_chat_member,
        new_chat_member,
    },
    command::common_command_handler,
};
use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    prelude::Dispatcher,
};
use teloxide_core::{
    adaptors::DefaultParseMode,
    prelude::RequesterExt,
    types::{
        ChatMemberUpdated,
        ParseMode,
        Update,
    },
};

pub type Bot = DefaultParseMode<teloxide::Bot>;

pub async fn run() {
    let bot = teloxide::Bot::from_env().parse_mode(ParseMode::Html);
    let handler = dptree::entry()
        .inspect(|_u: Update| {
            //eprintln!("Update: {u:#?}");
        })
        .branch(Update::filter_message().endpoint(common_command_handler))
        .branch(
            Update::filter_chat_member()
                .branch(
                    dptree::filter(|m: ChatMemberUpdated| {
                        m.old_chat_member.is_left() && m.new_chat_member.is_present()
                    })
                    .endpoint(new_chat_member),
                )
                .branch(
                    dptree::filter(|m: ChatMemberUpdated| {
                        m.old_chat_member.is_present() && m.new_chat_member.is_left()
                    })
                    .endpoint(left_chat_member),
                ),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
