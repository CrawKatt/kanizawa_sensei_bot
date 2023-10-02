use teloxide::utils::command::BotCommands;
use teloxide_core::requests::ResponseResult;
use teloxide_core::types::{Me, Message};
use crate::commands::common::{csharp, help, rust};
use crate::commands::fun::send;
use crate::enums::{BotCommonCommands, DocsCommands, FunCommands};
use crate::prelude::Bot;

pub async fn handler(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    let Some(text) = msg.text() else {
        return Ok(())
    };

    // This is Cursed LOL
    match BotCommands::parse(text, me.username()).as_ref() {
        Ok(BotCommonCommands::Start) => rust(bot, msg).await?,
        Ok(BotCommonCommands::Help) => help(bot, msg).await?,
        _ => {
            match BotCommands::parse(text, me.username()) {
                Ok(DocsCommands::Rust) => rust(bot, msg).await?,
                Ok(DocsCommands::Csharp) => csharp(bot, msg).await?,
                _ => {
                    let Ok(FunCommands::Send) = BotCommands::parse(text, me.username()) else {
                        return Ok(())
                    };
                    send(bot, msg).await?;
                },
            }
        },
    }

    Ok(())

}

pub async fn for_database(msg: Message) -> ResponseResult<()> {
    let Some(_) = msg.text() else {
        return Ok(())
    };

    //insert_user_to_sql(&msg).await?;

    Ok(())
}