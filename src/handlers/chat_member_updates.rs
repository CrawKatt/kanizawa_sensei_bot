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

const BANNED_CHARS: [&str; 42] = [
    "Б", "Г", "Д", "Ё", "Ж", "З",
    "И", "Й", "Л", "П", "Ф", "Ц",
    "Ч", "Ш", "Щ", "Ъ", "Ы", "Ь",
    "Э", "Ю", "Я", "б", "г", "д",
    "ё", "ж", "з", "и", "й", "л",
    "п", "ф", "ц", "ч", "ш", "щ",
    "ъ", "ы", "ь", "э", "ю", "я",
];

pub async fn new_chat_member(
    bot: Bot,
    chat_member: ChatMemberUpdated,
) -> ResponseResult<()> {
    let user = chat_member.old_chat_member.user;
    let user_id = user.id;
    let first_name = &user.first_name;
    let telegram_group_name = chat_member.chat.title().unwrap_or("");
    let ChatId(user_id) = ChatId::from(user_id);
    let username = user.mention()
        .unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

    if first_name.len() >= 3 && BANNED_CHARS.iter().any(|&banned_char| first_name.contains(banned_char)) {
        bot.ban_chat_member(chat_member.chat.id, user.id).await?;
        bot.send_message(
            chat_member.chat.id,
            format!("{username} Contiene caracteres Rusos, \nAccion: baneado!"),
        ).await?;
        return Ok(())
    }

    bot.send_message(
        chat_member.chat.id,
        format!("Bienvenido a {telegram_group_name} {username}!"),
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

    bot.send_message(chat_member.chat.id, format!("Hasta pronto {username}!"))
        .await?;

    Ok(())
}
