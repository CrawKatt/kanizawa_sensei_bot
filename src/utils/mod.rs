pub mod db;

use crate::prelude::Bot;
use std::time::Duration;
use teloxide_core::{
    requests::ResponseResult,
    prelude::{
        ChatId,
        Requester,
    },
    types::{
        Message,
        MessageId,
        ChatMember,
        ChatMemberStatus,
        User,
    },
};
use tokio::time::sleep;
use crate::utils::db::send_data;

pub trait AdminOrOwner {
    fn is_admin(&self) -> bool;
    fn is_owner(&self) -> bool;
    fn is_admin_or_owner(&self) -> bool;
}

impl AdminOrOwner for ChatMember {
    fn is_admin(&self) -> bool {
        self.status() == ChatMemberStatus::Administrator
    }

    fn is_owner(&self) -> bool {
        self.status() == ChatMemberStatus::Owner
    }

    fn is_admin_or_owner(&self) -> bool {
        self.is_admin() || self.is_owner()
    }
}

pub trait Timer {
    fn delete_message_timer(
        &self,
        bot: Bot,
        chat_id: ChatId,
        msg_id: MessageId,
        secs: u64,
    ) -> &Self;
}

impl Timer for Message {
    fn delete_message_timer(
        &self,
        bot: Bot,
        chat_id: ChatId,
        msg_id: MessageId,
        secs: u64,
    ) -> &Self {
        let ok_or_err = self.id;
        tokio::spawn(async move {
            sleep(Duration::from_secs(secs)).await;
            bot.delete_message(chat_id, ok_or_err)
                .await
                .unwrap_or_default();
            bot.delete_message(chat_id, msg_id)
                .await
                .unwrap_or_default();
        });

        self
    }
}

#[async_trait::async_trait]
pub trait MessageExt {
    async fn parse_id(&self) -> u64;
    fn extract_first_new_member<'user>(&'user self, msg: &'user Message) -> Option<&User>;
}

#[async_trait::async_trait]
impl MessageExt for Message {
    /// # Parse the message to get the user_id
    /// Parse the message to get the user_id from:
    /// - Reply to a Message
    /// - Send a Message with @username or user_id as an argument (e.g. /ban @username, /ban 12345678)
    async fn parse_id(&self) -> u64 {
        let Some(replied) = self.reply_to_message() else {
            self.text().unwrap_or_default().parse::<u64>().unwrap_or_default();
            return send_data(self.clone()).await.unwrap_or_default();
        };
        replied.text().unwrap_or_default().parse::<u64>().unwrap_or_default()
    }

    fn extract_first_new_member<'user>(&'user self, msg: &'user Message) -> Option<&User> {
        msg.reply_to_message()?.new_chat_members()?.first()
    }
}

pub async fn get_user_data_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(replied) = msg.reply_to_message() else {
        bot.send_message(msg.chat.id, "❌ No has respondido a ningún mensaje para obtener los datos del usuario").await?;
        return Ok(())
    };

    let data = bot.get_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
    bot.send_message(msg.chat.id, format!("{data:#?}")).await?;

    Ok(())
}