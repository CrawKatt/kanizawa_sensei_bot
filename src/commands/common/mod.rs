use teloxide_core::prelude::Requester;
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::Message;
use teloxide_core::types::ParseMode::MarkdownV2;
use teloxide_core::payloads::SendMessageSetters;
use crate::prelude::Bot;
use crate::utils::Timer;

pub async fn rust(bot: Bot, msg: Message) -> ResponseResult<()> {
    let concept = msg.text()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .get(1..)
        .map_or(
            String::new(),
            |s| s.join(" ")
        );

    let path = format!("docs/rust/{concept}.md");

    if let Ok(data) = std::fs::read_to_string(&path) {
        let ok = bot.send_message(msg.chat.id, data).parse_mode(MarkdownV2).await?;
        ok.delete_message_timer(bot, msg.chat.id, ok.id, msg.id, 5);
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
        .map_or(
            String::new(), |s| s.join(" ")
        );

    let path = format!("docs/csharp/{concept}.md");

    if let Ok(data) = std::fs::read_to_string(&path) {
        let ok = bot.send_message(msg.chat.id, data).parse_mode(MarkdownV2).await?;
        ok.delete_message_timer(bot, msg.chat.id, ok.id, msg.id, 5);
    }

    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    let ok = bot.send_message(msg.chat.id, "Todo").await?;
    ok.delete_message_timer(bot, msg.chat.id, ok.id, msg.id, 5);

    Ok(())
}