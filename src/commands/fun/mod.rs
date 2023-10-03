use std::str::FromStr;
use teloxide::utils::html;
use teloxide_core::prelude::{ChatId, Requester};
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::{InputFile, Message, ParseMode};
use teloxide_core::payloads::SendAnimationSetters;
use teloxide_core::payloads::SendMessageSetters;
use crate::utils::Timer;
use crate::prelude::Bot;

pub async fn send(bot: Bot, msg: Message) -> ResponseResult<()> {

    let Some(text) = msg.text() else {
        return Ok(())
    };

    let Some(user) = msg.from() else {
        return Ok(())
    };

    let user_id = user.id;
    let ChatId(user_id) = ChatId::from(user_id);
    let parts: Vec<&str> = text
        .split_whitespace()
        .collect();

    if parts.len() < 3 {
        bot.send_message(msg.chat.id,"Uso: /send `<action> <user>`")
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        return Ok(())
    }

    let action = parts[1];
    let category_result = nekosbest::Category::from_str(action);
    let Ok(category) = category_result else {
        return Ok(())
    };

    let random_gif = nekosbest::get(category).await;
    let username_target = parts[2];
    let username_author = user
        .mention()
        .unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

    let message = match action {
        "kiss" => format!("{username_author} besó a {username_target}"),
        "cuddle" => format!("{username_author} envuelve a {username_target} con un fuerte abrazo"),
        "hug"  => format!("{username_author} abrazó a {username_target}"),
        "pat" => format!("{username_author} acarició a {username_target}"),
        "slap" => format!("{username_author} cacheteó a {username_target}"),
        "kick" => format!("{username_author} pateó a {username_target}"),
        "punch" => format!("{username_author} le dio un puñetazo a {username_target}"),
        "shoot" => format!("{username_author} le disparó a {username_target}"),
        "yeet" => format!("{username_author} mandó a {username_target} a la punta del cerro"),
        _ => format!("{action} no es una categoria valida"),
    };

    let random_gif = random_gif.map_err(|_| ());

    let Ok(random_gif) = random_gif else {
        return Ok(())
    };

    let gif_url = random_gif
        .url
        .parse();

    let Ok(url) = gif_url else {
        return Ok(())
    };

    let ok = bot.send_animation(msg.chat.id, InputFile::url(url))
        .caption(message)
        .parse_mode(ParseMode::Html)
        .await?;

    ok.delete_message_timer(bot, msg.chat.id, ok.id, msg.id, 60);

    Ok(())
}