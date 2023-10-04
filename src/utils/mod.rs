use crate::prelude::Bot;
use std::time::Duration;
use teloxide_core::{
    prelude::{
        ChatId,
        Requester,
    },
    types::{
        Message,
        MessageId,
    },
};
use teloxide_core::types::{ChatMember, ChatMemberStatus};
use tokio::time::sleep;

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

pub trait MessageExt {
    fn parse_id(&self) -> u64;
}

impl MessageExt for Message {
    fn parse_id(&self) -> u64 {
        self.text().unwrap().split_once(' ').map(|(_, a)| a.trim().parse::<u64>().unwrap_or_default()).unwrap()
    }
}