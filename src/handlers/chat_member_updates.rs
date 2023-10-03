use crate::prelude::Bot;
use teloxide::utils::html;
use teloxide_core::{
    prelude::{
        ChatId,
        Requester,
    },
    requests::ResponseResult,
    types::ChatMemberUpdated,
};

pub async fn new_chat_member(
    bot: Bot,
    chat_member: ChatMemberUpdated,
) -> ResponseResult<()> {
    let user = chat_member.old_chat_member.user.clone();

    let user_id = user.id;

    let telegram_group_name = chat_member.chat.title().unwrap_or("");

    let ChatId(user_id) = ChatId::from(user_id);

    let username = user
        .mention()
        .unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

    bot.send_message(
        chat_member.chat.id,
        format!("Welcome to {telegram_group_name} {username}!"),
    )
    .await?;

    Ok(())
}

pub async fn left_chat_member(
    bot: Bot,
    chat_member: ChatMemberUpdated,
) -> ResponseResult<()> {
    let user = chat_member.old_chat_member.user;

    let user_id = user.id;

    let ChatId(user_id) = ChatId::from(user_id);

    let username = user
        .mention()
        .unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

    bot.send_message(chat_member.chat.id, format!("Goodbye {username}!"))
        .await?;

    Ok(())
}
