use crate::{prelude::Bot, utils::Timer};
use teloxide_core::{payloads::SendMessageSetters, prelude::Requester, requests::ResponseResult};
use teloxide_core::types::{Message, ParseMode::MarkdownV2};
use teloxide_core::prelude::UserId;
use crate::handlers::buttons::create_button;
use crate::handlers::buttons::help_action;
use crate::utils::MessageExt;

pub async fn handle_docs(
    bot: Bot,
    msg: Message,
    language: &str,
) -> ResponseResult<()> {
    let concept = msg
        .text()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .get(1..)
        .map_or(String::new(), |s| s.join(" "));

    let path = format!("docs/{language}/{concept}.md");

    let Ok(data) = std::fs::read_to_string(&path) else {
        bot.send_message(msg.chat.id, format!("Uso: /{language} \\<concepto\\>"))
            .reply_to_message_id(msg.id)
            .parse_mode(MarkdownV2)
            .await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);

        return Ok(())
    };

    bot.send_message(msg.chat.id, data)
        .reply_to_message_id(msg.id)
        .parse_mode(MarkdownV2)
        .await?
        .delete_message_timer(bot, msg.chat.id, msg.id, 60);

    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    help_action(bot, msg).await?;

    Ok(())
}

pub async fn start(bot: Bot, msg: Message) -> ResponseResult<()> {
    create_button(bot, msg).await?;

    Ok(())
}

pub async fn info(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Info for @username/user_id (/info @username) (/info 12345678)
    let Some(replied) = msg.reply_to_message() else {
        let parsed_id = msg.parse_id().await;
        let info = bot.get_chat_member(msg.chat.id, UserId(parsed_id)).await?;
        let first_name = info.user.first_name;
        let last_name = info.user.last_name.unwrap_or_else(|| "Ninguno".to_owned());
        let username = info.user.username.unwrap_or_default();
        let user_id = info.user.id;
        let user_info = format!("Nombre: {first_name} \nApellido: {last_name} \nUsername: @{username} \nuser_id: <code>{user_id}</code>");

        bot.send_message(msg.chat.id, user_info)
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 60);

        return Ok(())
    };

    // Info for reply to a message (/info <replying message>)
    // panic if replied a remove join event (e.g "User removed Target)
    let from = replied.from().unwrap_or_else(|| replied.new_chat_members().unwrap_or_default().first().unwrap());
    let info = bot.get_chat_member(msg.chat.id, from.id).await?;

    let first_name = info.user.first_name;
    let last_name = info.user.last_name.unwrap_or_else(|| "Ninguno".to_owned());
    let username = info.user.username.unwrap_or_default();
    let user_id = info.user.id;
    let user_info = format!("Nombre: {first_name} \nApellido: {last_name} \nUsername: @{username} \nuser_id: <code>{user_id}</code>");

    bot.send_message(msg.chat.id, user_info)
        .reply_to_message_id(msg.id).await?
        .delete_message_timer(bot, msg.chat.id, msg.id, 60);

    Ok(())
}