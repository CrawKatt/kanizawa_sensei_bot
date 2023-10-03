use crate::{
    commands::{
        common::{
            csharp,
            help,
            rust,
        },
        fun::send,
    },
    enums::{
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

pub async fn common_command_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> ResponseResult<()> {
    let Some(text) = msg.text() else {
        return Ok(())
    };

    match BotCommands::parse(text, me.username()) {
        Ok(BotCommonCommands::Start) => rust(bot, msg).await?,
        Ok(BotCommonCommands::Help) => help(bot, msg).await?,
        _ => docs_command_handler(bot, msg, me).await?,
    }

    Ok(())
}

pub async fn docs_command_handler(
    bot: Bot,
    msg: Message,
    me: Me
) -> ResponseResult<()> {
    let Some(text) = msg.text() else {
        return Ok(())
    };

    match BotCommands::parse(text, me.username()) {
        Ok(DocsCommands::Rust) => rust(bot, msg).await?,
        Ok(DocsCommands::Csharp) => csharp(bot, msg).await?,
        _ => fun_command_handler(bot, msg, me).await?,
    };

    Ok(())
}

pub async fn fun_command_handler(
    bot: Bot,
    msg: Message,
    me: Me
) -> ResponseResult<()> {
    let Some(text) = msg.text() else {
        return Ok(())
    };

    let Ok(FunCommands::Send) = BotCommands::parse(text, me.username()) else {
        for_database(msg).await?;
        return Ok(())
    };
    send(bot, msg).await?;

    Ok(())
}

pub async fn for_database(msg: Message) -> ResponseResult<()> {
    let Some(_) = msg.text() else { return Ok(()) };

    // insert_user_to_sql(&msg).await?;

    Ok(())
}
