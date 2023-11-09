use teloxide::utils::command::BotCommands;
use teloxide_core:: types::{Me, Message};
use teloxide_core::requests::ResponseResult;

use crate::prelude::Bot;
use crate::commands::common::{info, report};
use crate::utils::db::get_user_data;
use crate::enums::{AdminCommands, BotCommonCommands, DocsCommands, FunCommands};
use crate::commands::admin::{ban::banning, unban::unbanning, mute::muting, unmute::unmuting};
use crate::commands::common::{handle_docs, start, help};
use crate::commands::fun::send;

pub async fn common_command_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    match BotCommands::parse(text, me.username()) {
        Ok(BotCommonCommands::Start) => start(bot, msg).await?,
        Ok(BotCommonCommands::Help) => help(bot, msg).await?,
        Ok(BotCommonCommands::Info) => info(bot, msg).await?,
        Ok(BotCommonCommands::Report) => report(bot, msg).await?,
        _ => docs_command_handler(bot, msg, me).await?,
    }

    Ok(())
}

pub async fn docs_command_handler(
    bot: Bot,
    msg: Message,
    me: Me
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    match BotCommands::parse(text, me.username()) {
        Ok(DocsCommands::Rust) => handle_docs(bot, msg, "rust").await?,
        Ok(DocsCommands::Csharp) => handle_docs(bot, msg, "csharp").await?,
        Ok(DocsCommands::Python) => handle_docs(bot, msg, "python").await?,
        _ => Box::pin(admin_command_handler(bot, msg, me)).await?,
    };

    Ok(())
}

pub async fn admin_command_handler(
    bot: Bot,
    msg: Message,
    me: Me
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    match BotCommands::parse(text, me.username()) {
        Ok(AdminCommands::Ban) => banning(bot, msg).await?,
        Ok(AdminCommands::Unban) => unbanning(bot, msg).await?,
        Ok(AdminCommands::Mute) => muting(bot, msg).await?,
        Ok(AdminCommands::Unmute) => unmuting(bot, msg).await?,
        _ => fun_command_handler(bot, msg, me).await?
    }

    Ok(())
}

pub async fn fun_command_handler(
    bot: Bot,
    msg: Message,
    me: Me
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    let Ok(FunCommands::Send) = BotCommands::parse(text, me.username()) else {
        get_user_data(bot, msg).await.unwrap_or_else(|e| {
            println!("Error al obtener los datos del usuario \n{e:#?}");
        });
        return Ok(())
    };
    send(bot, msg).await?;

    Ok(())
}