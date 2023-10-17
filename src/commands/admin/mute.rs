use teloxide_core::{
    payloads::SendMessageSetters,
    prelude::{
        Requester,
        UserId
    },
    requests::ResponseResult,
    types::Message
};
use teloxide_core::types::ChatPermissions;
use crate::error::{PermissionsDenied, handle_status};
use crate::prelude::Bot;
use crate::utils::{MessageExt, Timer};

pub async fn muting(bot: Bot, msg: Message) -> ResponseResult<()> {
    let user_status = handle_status(&bot, &msg).await;
    if !user_status {
        bot.send_message(msg.chat.id, PermissionsDenied)
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);
        return Ok(())
    }

    // Necesario para el Mute por id (/mute 12345678)
    let Some(replied) = msg.reply_to_message() else {
        let parsed_id = msg.parse_id().await;
        bot.restrict_chat_member(msg.chat.id, UserId(parsed_id),ChatPermissions::empty()).await?;
        bot.send_message(msg.chat.id, "✅ Usuario silenciado")
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);
        return Ok(())
    };

    // Mute for reply to a Join Event (/mute <user> Joined the group)
    let Some(user) = replied.from() else {
        let Some(user) = msg.extract_first_new_member(&msg) else {
            return Ok(())
        };
        bot.restrict_chat_member(msg.chat.id, user.id,ChatPermissions::empty()).await?;
        bot.send_message(msg.chat.id, "✅ Se ha removido el silencio al usuario")
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);

        return Ok(())
    };

    // Mute for reply to a message (/mute <replying message>)
    let user_id = user.id;
    let username = user.username
        .as_ref()
        .map_or_else(String::new, std::string::ToString::to_string);

    bot.restrict_chat_member(msg.chat.id, user_id,ChatPermissions::empty()).await?;
    bot.send_message(msg.chat.id, format!("✅ @{username} [<code>{user_id}</code>] ha sido silenciado"))
        .reply_to_message_id(msg.id).await?.delete_message_timer(bot, msg.chat.id, msg.id, 60);

    Ok(())
}