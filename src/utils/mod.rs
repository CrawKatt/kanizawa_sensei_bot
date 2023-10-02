use std::time::Duration;
use teloxide_core::prelude::{ChatId, Requester};
use teloxide_core::types::{Message, MessageId};
use tokio::time::sleep;
use crate::prelude::Bot;

pub trait Timer {
    fn delete_message_timer(
        &self,
        bot: Bot,
        chat_id: ChatId,
        ok_or_err: MessageId,
        msg_id: MessageId,
        secs: u64
    );
}

impl Timer for Message {
    fn delete_message_timer(
        &self,
        bot: Bot,
        chat_id: ChatId,
        ok_or_err: MessageId,
        msg_id: MessageId,
        secs: u64
    ) {
        tokio::spawn(async move {
            sleep(Duration::from_secs(secs)).await;
            bot.delete_message(chat_id, ok_or_err)
                .await
                .unwrap_or_default();
            bot.delete_message(chat_id, msg_id)
                .await
                .unwrap_or_default();
        });
    }
}