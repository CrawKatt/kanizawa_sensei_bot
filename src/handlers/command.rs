use crate::{
    commands::{
        common::{
            csharp,
            help,
            rust,
        },
        fun::send,
        admin::{
            ban::banning,
            unban::unbanning,
            mute::muting,
            unmute::unmuting,
        },
    },
    enums::{
        AdminCommands,
        BotCommonCommands,
        DocsCommands,
        FunCommands,
    },
    prelude::Bot,
};
use teloxide::utils::command::BotCommands;
use teloxide_core::{
    requests::ResponseResult,
    types::{
        Me,
        Message,
    },
};
use crate::utils::db::get_user_data;

pub async fn common_command_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> ResponseResult<()> {
    let text = msg.text().unwrap_or_default();
    match BotCommands::parse(text, me.username()) {
        Ok(BotCommonCommands::Start) => rust(bot, msg).await?,
        Ok(BotCommonCommands::Help) => help(bot, msg).await?,
        _ => Box::pin(docs_command_handler(bot, msg, me)).await?,
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
        Ok(DocsCommands::Rust) => rust(bot, msg).await?,
        Ok(DocsCommands::Csharp) => csharp(bot, msg).await?,
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
        for_database(msg).await?;
        return Ok(())
    };
    send(bot, msg).await?;

    Ok(())
}

pub async fn for_database(msg: Message) -> ResponseResult<()> {
    let Some(_) = msg.text() else { return Ok(()) };
    get_user_data(msg).await.unwrap_or_else(|e| {
        println!("Error al obtener los datos del usuario \n{e:#?}");
    });

    Ok(())
}