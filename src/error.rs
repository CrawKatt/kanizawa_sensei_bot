use teloxide_core::payloads::SendMessageSetters;
pub use ErrMessage::*;
use teloxide_core::prelude::Requester;
use teloxide_core::types::{Message, UserId};
use crate::prelude::Bot;
use crate::utils::{AdminOrOwner, MessageExt};

pub enum ErrMessage {
    /*
    AlreadyBanned,
    AlreadyMuted,
    NotBanned,
    */
    NotMuted,
    /*
    Unmuted,
    NotIdProvided404,
    */
    IdOrUsernameNotValid,
    /*
    NotUsernameFound404,
    */
    PermissionsDenied,
    //UserNotFound,
}

impl std::fmt::Display for ErrMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = self.message();
        write!(f, "{message}")
    }
}

impl ErrMessage {
    const fn message(&self) -> &str {
        match self {
            /*
            Self::AlreadyBanned => "Ya está baneado. Usa este comando solo para banear a alguien que no haya sido baneado antes",
            Self::AlreadyMuted => "Ya está silenciado. Usa este comando solo para silenciar a alguien que no haya sido silenciado antes",
            Self::NotBanned => "No está baneado. Usa este comando solo para remover el Ban de alguien que ya haya sido baneado",
            */
            NotMuted => "No está silenciado. Usa este comando solo para remover el silencio de alguien que ya haya sido silenciado",
            /*
            Unmuted => "Ya no está silenciado.",
            Self::NotIdProvided404 => "❌ No has especificado un ID para obtener el usuario",
            */
            IdOrUsernameNotValid => "❌ El ID o @Username proporcionado no es válido, considera reenviar un mensaje al bot para hacer un ban por ID",
            /*
            Self::NotUsernameFound404 => "❌ No se encontró ningún usuario con el username",
            */
            PermissionsDenied => "❌ No tienes permisos para usar este comando",
            //Self::UserNotFound => "❌ No se pudo obtener el usuario",
        }
    }
}

impl From<ErrMessage> for String {
    fn from(err_message: ErrMessage) -> Self {
        err_message.message().to_string()
    }
}

pub async fn handle_status(bot: &Bot, msg: &Message) -> bool {
    let chat_id = msg.chat.id;

    let Some(replied) = msg.from() else {
        return false
    };

    let chat_member = bot.get_chat_member(chat_id, replied.id).await;
    chat_member.expect("ERROR").is_admin_or_owner()
}

pub async fn handle_target_mute(bot: &Bot, msg: &Message) -> bool {
    let chat_id = msg.chat.id;

    let Some(replied) = msg.from() else {
        let parsed_id = msg.parse_id().await;
        if parsed_id == 0 {
            bot.send_message(chat_id, IdOrUsernameNotValid)
                .reply_to_message_id(msg.id).await
                .expect("ERROR");
            return false
        }
        let chat_member = bot.get_chat_member(chat_id, UserId(parsed_id)).await;

        if chat_member.as_ref().expect("ERROR").is_restricted() {
            return true
        }

        return false
    };

    let chat_member = bot.get_chat_member(chat_id, replied.id).await;
    chat_member.expect("ERROR").is_restricted()
}

pub async fn handle_target_ban(bot: &Bot, msg: &Message) -> bool {
    let chat_id = msg.chat.id;

    let Some(replied) = msg.from() else {
        let parsed_id = msg.parse_id().await;
        if parsed_id == 0 {
            bot.send_message(chat_id, IdOrUsernameNotValid)
                .reply_to_message_id(msg.id).await
                .expect("ERROR");
            return false
        }
        let chat_member = bot.get_chat_member(chat_id, UserId(parsed_id)).await;

        if chat_member.as_ref().expect("ERROR").is_banned() {
            return true
        }

        return false
    };

    let chat_member = bot.get_chat_member(chat_id, replied.id).await;
    chat_member.expect("ERROR").is_restricted()
}


/*
pub enum Formatter<'t> {
    Code(&'t str),
}

impl<'t> std::fmt::Display for Formatter<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Formatter::Code(text) => write!(f, "[<code>{text}</code>]"),
        }
    }
}
*/