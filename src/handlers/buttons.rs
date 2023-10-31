use teloxide_core::prelude::{Requester, UserId};
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message};
use crate::prelude::Bot;
use teloxide_core::payloads::SendMessageSetters;
use teloxide_core::payloads::EditMessageTextSetters;

const GITHUB: &str = "GitHub: \nhttps://github.com/CrawKatt \
                    \n\nRepositorio del Bot: \nhttps://github.com/CrawKatt/kanizawa_sensei_bot";

pub async fn create_button(bot: Bot, msg: Message) -> ResponseResult<()> {
    // Create a list of buttons and send them.
    let keyboard = make_main_keyboard();

    bot.send_message(
        msg.chat.id,
        "Hola, soy un Bot que administra grupos de Telegram y seré tu asistente personal en tu \
         camino aprendiendo a Programar\\.",
    )
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub fn make_generic_keyboard(options: &[&str], add_back_button: bool) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for option in options.chunks(3) {
        let row = option
            .iter()
            .map(|&option| {
                let callback_data = if option == "Volver" && add_back_button {
                    "back_to_main_keyboard".to_owned()
                } else {
                    option.to_owned()
                };

                InlineKeyboardButton::callback(option.to_owned(), callback_data)
            })
            .collect();

        keyboard.push(row);
    }

    if add_back_button {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "Volver".to_owned(),
            "back_to_main_keyboard".to_owned(),
        )]);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn help_action(bot: Bot, msg: Message) -> ResponseResult<()> {
    let keyboard = make_main_keyboard();

    bot.send_message(
        msg.chat.id,
        "¿Necesitas ayuda? Prueba alguna de las opciones disponibles:",
    )
        .reply_markup(keyboard)
        .await?;

    Ok(())
}
/*
pub fn make_unban_keyboard() -> InlineKeyboardMarkup {
    let callback_data = "Desbanear".to_owned();
    let keyboard = vec![vec![InlineKeyboardButton::callback("Desbanear", callback_data)]];
    InlineKeyboardMarkup::new(keyboard)
}
*/

pub fn make_unmute_keyboard(user_id: UserId) -> InlineKeyboardMarkup {
    let callback_data = format!("Desbanear_{user_id}");
    println!("{callback_data:#?}");

    let keyboard = vec![vec![InlineKeyboardButton::callback("Desmutear", callback_data)]];
    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_main_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let options = [
        "Ajustes",
        "Donar",
        "Acerca de",
        "Comandos",
        "Languages",
        "Ayuda",
    ];

    for option in options.chunks(3) {
        let row = option
            .iter()
            .map(|&option| {
                InlineKeyboardButton::callback(option.to_owned(), option.to_owned())
            })
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_settings_keyboard() -> InlineKeyboardMarkup {
    make_generic_keyboard(&["Idioma", "Anti-Spam", "Advertencias", "Reglamento"], true)
}

pub fn make_about_keyboard() -> InlineKeyboardMarkup {
    make_generic_keyboard(&["GitHub"], true)
}

pub fn make_donate_keyboard() -> InlineKeyboardMarkup {
    make_generic_keyboard(&["PayPal", "Patreon", "Crypto"], true)
}

pub fn make_language_keyboard() -> InlineKeyboardMarkup {
    make_generic_keyboard(&["Español", "English", "日本語"], true)
}

pub fn make_back_button_keyboard() -> InlineKeyboardMarkup {
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    InlineKeyboardMarkup::new(keyboard)
}
/*
fn extract_user_id_from_callback_data(callback_data: &str) -> UserId {
    let parts: Vec<&str> = callback_data.split('_').collect();
    if parts.len() == 2 {
        if let Ok(user_id) = parts[1].parse() {
            println!("user_id = {}", user_id);
            return UserId(user_id);
        }
    }
    // Si no se pudo extraer el user_id, puedes devolver un valor predeterminado o manejar el error de otra manera.
    UserId(404) // Valor predeterminado
}
*/
pub async fn callback_handler(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {
    let (text, keyboard) = match query.data.as_deref() {
        Some("Ajustes" | "Donar" | "Acerca de" | "Ayuda" | "Languages" | "back_to_main_keyboard") => (
            "Elige una opción:".to_owned(),
            match query.data.as_deref() {
                Some("Ajustes") => make_settings_keyboard(),
                Some("Donar") => make_donate_keyboard(),
                Some("Acerca de") => make_about_keyboard(),
                Some("Languages") => make_language_keyboard(),
                _ => make_main_keyboard(),
            },
        ),
        Some("GitHub") => (
            GITHUB.to_owned(),
            make_back_button_keyboard(),
        ),
        Some("Desmutear") => {
            let user_id = query.from.id;
            (
                "✅ Se ha removido el silencio al usuario".to_owned(),
                make_unmute_keyboard(user_id),
            )
        },
        _ => return Ok(()),
    };

    let Some(Message { id, ref chat, .. }) = query.message else {
        return Ok(())
    };

    bot.edit_message_text(chat.id, id, text)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}