use crate::{
    prelude::Bot,
    utils::Timer,
};
use teloxide_core::{
    payloads::SendMessageSetters,
    prelude::Requester,
    requests::ResponseResult,
    types::{
        Message,
        ParseMode::MarkdownV2,
    },
};

pub async fn rust(bot: Bot, msg: Message) -> ResponseResult<()> {
    let concept = msg.text()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .get(1..)
        .map_or(String::new(), |s| s.join(" "));

    let path = format!("docs/rust/{concept}.md");

    if let Ok(data) = std::fs::read_to_string(&path) {
        bot.send_message(msg.chat.id, data)
            .parse_mode(MarkdownV2)
            .await?.delete_message_timer(bot, msg.chat.id, msg.id, 60);
    }

    Ok(())
}

pub async fn csharp(bot: Bot, msg: Message) -> ResponseResult<()> {
    let concept = msg
        .text()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .get(1..)
        .map_or(String::new(), |s| s.join(" "));

    let path = format!("docs/csharp/{concept}.md");

    if let Ok(data) = std::fs::read_to_string(path) {
        bot.send_message(msg.chat.id, data)
            .parse_mode(MarkdownV2)
            .await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 60);
    }

    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Todo").await?
        .delete_message_timer(bot, msg.chat.id, msg.id, 5);

    Ok(())
}