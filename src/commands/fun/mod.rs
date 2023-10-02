use std::str::FromStr;
use teloxide::utils::html;
use teloxide_core::prelude::{ChatId, Requester};
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::{InputFile, Message, ParseMode};
use crate::prelude::Bot;
use teloxide_core::payloads::SendAnimationSetters;
use teloxide_core::payloads::SendMessageSetters;

pub async fn send(bot: Bot, msg: Message) -> ResponseResult<()> {

    let Some(text) = msg.text() else {
        return Ok(())
    };

    let user = msg.from().unwrap();

    let user_id = user.id;

    let ChatId(user_id) = ChatId::from(user_id);

    let parts: Vec<&str> = text.split_whitespace().collect();

    if parts.len() < 3 {
        bot.send_message(msg.chat.id,"Uso: /send `<action> <user>`").parse_mode(ParseMode::MarkdownV2).await?;
        return Ok(())
    }

    let action = parts[1];

    let category_result = nekosbest::Category::from_str(action);

    let random_gif = match category_result {
        Ok(category) => nekosbest::get(category).await,
        Err(error) => {
            println!("Error: {error}");
            return Ok(())
        }
    };

    let username_target = parts[2];

    let username_author = user.mention().unwrap_or_else(|| html::user_mention(user_id, user.full_name().as_str()));

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
        _ => {
            println!("{action} no es una categoria valida");
            return Ok(())
        }
    };

    let random_gif = if let Ok(gif) = random_gif {
        Some(gif)
    } else {
        println!("No se pudo obtener un gif de la categoria {action}", );
        return Ok(())
    };

    bot.send_animation(msg.chat.id, InputFile::url(random_gif.unwrap().url.parse().unwrap()))
        .caption(message)
        .parse_mode(ParseMode::Html)
        .await?;

    /*
    let Some(action) = msg.text() else {
        return Ok(())
    };

    let (_, username_target) = action
        .find(' ')
        .map_or(
            ("", action), |index| action.split_at(index)
        );

    let username_author = msg
        .from()
        .as_ref()
        .and_then(|user| user.username.as_ref())
        .map_or("", |username| username);

    let category_result = nekosbest::Category::from_str(&action);

    let random_gif = match category_result {
        Ok(category) => nekosbest::get(category).await,
        Err(error) => {
            println!("Error: {}", error);
            return Ok(())
        }
    };
    */

    Ok(())
}