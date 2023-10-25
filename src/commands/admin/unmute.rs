use teloxide_core::{
    payloads::SendMessageSetters,
    requests::ResponseResult,
    prelude::{
        Requester,
        UserId
    },
    types::{
        Message,
        ChatPermissions
    }
};
use crate::prelude::Bot;
use crate::error::{PermissionsDenied, handle_status, IdOrUsernameNotValid, handle_target_mute, NotMuted};
use crate::utils::{MessageExt, Timer};

pub async fn unmuting(bot: Bot, msg: Message) -> ResponseResult<()> {
    let user_status = handle_status(&bot, &msg).await;
    let target_status = handle_target_mute(&bot, &msg).await;
    if !target_status {
        bot.send_message(msg.chat.id, NotMuted)
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);
        return Ok(())
    }

    if !user_status {
        bot.send_message(msg.chat.id, PermissionsDenied)
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);
        return Ok(())
    }

    // Necesario para el Unmute por id (/unmute 12345678)
    let Some(replied) = msg.reply_to_message() else {
        let parsed_id = msg.parse_id().await;
        if parsed_id == 404 {
            bot.send_message(msg.chat.id, IdOrUsernameNotValid)
                .reply_to_message_id(msg.id).await?
                .delete_message_timer(bot, msg.chat.id, msg.id, 10);
            return Ok(())
        }
        bot.restrict_chat_member(msg.chat.id, UserId(parsed_id),ChatPermissions::all()).await?;
        bot.send_message(msg.chat.id, "✅ Usuario silenciado")
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);
        return Ok(())
    };

    // Unmute for reply to a Join Event (/unmute <user> Joined the group)
    let Some(user) = replied.from() else {
        let Some(user) = msg.extract_first_new_member(&msg) else {
            return Ok(())
        };
        bot.restrict_chat_member(msg.chat.id, user.id,ChatPermissions::all()).await?;
        bot.send_message(msg.chat.id, format!("✅ Se ha removido el silencio a {:?}", user.mention()))
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);

        return Ok(())
    };

    // Unmute for reply to a message (/unmute <replying message>)
    let user_id = user.id;
    let username = user.username
        .as_ref()
        .map_or_else(String::new, std::string::ToString::to_string);

    bot.restrict_chat_member(msg.chat.id, user_id,ChatPermissions::all()).await?;
    bot.send_message(msg.chat.id, format!("✅ @{username} [<code>{user_id}</code>] ha sido silenciado"))
        .reply_to_message_id(msg.id).await?.delete_message_timer(bot, msg.chat.id, msg.id, 60);

    Ok(())
}