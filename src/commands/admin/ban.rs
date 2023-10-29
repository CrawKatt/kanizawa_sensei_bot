use teloxide_core::payloads::SendMessageSetters;
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::Message;
use teloxide_core::prelude::{Requester, UserId};
use crate::error::{PermissionsDenied, handle_status, IdOrUsernameNotValid, handle_target_ban};
use crate::prelude::Bot;
use crate::utils::{MessageExt, Timer};
use crate::utils::db::delete_user_backup;

pub async fn banning(bot: Bot, msg: Message) -> ResponseResult<()> {
    let user_status = handle_status(&bot, &msg).await;
    let target_status = handle_target_ban(&bot, &msg).await;

    if target_status {
        bot.send_message(msg.chat.id, "❌ El usuario ya está baneado")
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

    // Necesario para el ban por id (/ban 12345678)
    let Some(replied) = msg.reply_to_message() else {
        // metodo parse_id() en utils/mod.rs sirve para extraer el id del
        // mensaje ya sea mediante el @username o mediante el user_id proporcionado en un mensaje
        let parsed_id = msg.parse_id().await;
        if parsed_id == 404 {
            bot.send_message(msg.chat.id, IdOrUsernameNotValid)
                .reply_to_message_id(msg.id).await?
                .delete_message_timer(bot, msg.chat.id, msg.id, 10);
            return Ok(())
        }

        bot.ban_chat_member(msg.chat.id, UserId(parsed_id)).await?;
        bot.send_message(msg.chat.id, "✅ Usuario baneado")
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);
        delete_user_backup(parsed_id.to_string()).unwrap_or_default();

        return Ok(())
    };

    // Ban for reply to a Join Event (/ban <user> Joined the group)
    let Some(user) = replied.from() else {
        let Some(user) = msg.extract_new_member_info(&msg) else {
            return Ok(())
        };

        bot.ban_chat_member(msg.chat.id, user.id).await?;
        bot.send_message(msg.chat.id, "✅ Usuario baneado")
            .reply_to_message_id(msg.id).await?
            .delete_message_timer(bot, msg.chat.id, msg.id, 10);

        return Ok(())
    };

    // Ban for reply to a message (/ban <replying message>)
    let user_id = user.id;
    let username = user.username
        .as_ref()
        .map_or_else(String::new, std::string::ToString::to_string);

    bot.ban_chat_member(msg.chat.id, user_id).await?;
    bot.send_message(msg.chat.id, format!("✅ @{username} [<code>{user_id}</code>] ha sido baneado"))
        .reply_to_message_id(msg.id).await?.delete_message_timer(bot, msg.chat.id, msg.id, 60);

    Ok(())
}