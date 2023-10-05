use teloxide_core::payloads::SendMessageSetters;
use teloxide_core::prelude::{Requester, UserId};
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::Message;
use crate::error::{USER_NOT_FOUND, NOT_BANNED};
use crate::prelude::Bot;
use crate::utils::{AdminOrOwner, MessageExt, Timer};

pub async fn unbanning(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let Some(replied) = msg.reply_to_message() else {
        bot.unban_chat_member(chat_id, UserId(msg.parse_id())).await?;
        bot.send_message(msg.chat.id, "✅ Usuario desbaneado")
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 60);
        return Ok(())
    };

    let Some(user) = replied.from() else {
        bot.send_message(msg.chat.id, USER_NOT_FOUND)
            .reply_to_message_id(msg.id)
            .await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 60);

        return Ok(())
    };

    let Some(from) = msg.from() else {
        return Ok(())
    };

    let user_id = user.id;

    let username = user.username
        .as_ref()
        .map_or_else(String::new, std::string::ToString::to_string);

    let is_admin_or_owner = bot.get_chat_member(msg.chat.id, from.id)
        .await?
        .is_admin_or_owner();

    let chat_member = bot.get_chat_member(msg.chat.id, user_id)
        .await?
        .status()
        .is_banned();

    if is_admin_or_owner && !chat_member {
        bot.send_message(msg.chat.id, format!("❌ @{username} [<code>{user_id}</code>] {NOT_BANNED}"))
            .await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);

        return Ok(())
    }

    bot.unban_chat_member(msg.chat.id, user_id).await?;
    bot.send_message(msg.chat.id, format!("✅ @{username} [<code>{user_id}</code>] ha sido baneado"))
        .reply_to_message_id(msg.id).await?.delete_message_timer(bot, msg.chat.id, msg.id, 60);

    Ok(())
}