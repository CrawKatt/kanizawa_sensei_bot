pub mod db;

use std::fs::File;
use std::io::{BufReader, Write};
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
        ChatMember,
        ChatMemberStatus,
        User,
    },
};
use tokio::time::sleep;
use crate::utils::db::{Data, DB, send_data, SurrealResult};

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

#[async_trait::async_trait]
pub trait MessageExt {
    async fn parse_id(&self) -> u64;
    fn extract_first_new_member<'user>(&'user self, msg: &'user Message) -> Option<&User>;
}

#[async_trait::async_trait]
impl MessageExt for Message {
    /// # Parse the message to get the user_id
    /// Parse the message to get the user_id from:
    /// - Reply to a Message
    /// - Send a Message with @username or user_id as an argument (e.g. /ban @username, /ban 12345678)
    async fn parse_id(&self) -> u64 {
        let Some(replied) = self.reply_to_message() else {
            self.text().unwrap_or_default().parse::<u64>().unwrap_or_default();
            return send_data(self.clone()).await.unwrap_or_default();
        };
        replied.text().unwrap_or_default().parse::<u64>().unwrap_or(404)
    }

    fn extract_first_new_member<'user>(&'user self, msg: &'user Message) -> Option<&User> {
        msg.reply_to_message()?.new_chat_members()?.first()
    }
}

async fn backup_data() -> SurrealResult<()> {
    // Seleccionar todos los datos de la tabla "users"
    let data: Vec<Data> = DB.select("users").await?;

    // Serializar los datos a JSON
    let serialized_data = serde_json::to_string_pretty(&data);

    // Crear el archivo de respaldo
    let mut file = File::create("backup.json").expect("❌ No se pudo crear el archivo de respaldo");

    // Escribir los datos serializados en el archivo de respaldo
    file.write_all(serialized_data.expect("Ocurrio un error").as_bytes()).expect("❌ No se pudo escribir en el archivo de respaldo");

    Ok(())
}

pub async fn load_data() -> SurrealResult<()> {
    DB.use_ns("teloxide-namespace").use_db("teloxide").await?;

    // Abrir el archivo de respaldo
    let file = File::open("backup.json").expect("❌ No se pudo abrir el archivo de respaldo");

    // Leer el archivo de respaldo
    let reader = BufReader::new(file);

    // Deserializar los datos del archivo de respaldo
    let data: Vec<Data> = serde_json::from_reader(reader).expect("❌ No se pudo deserializar los datos del archivo de respaldo");

    for item in &data {
        // Insertar los datos en la base de datos
        let _created: Vec<Data> = DB.create("users").content(item).await?;
        //println!("Usuarios pre-cargados: \n{created:#?}\n");
    }

    Ok(())
}

/*
pub async fn get_user_data_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let Some(replied) = msg.reply_to_message() else {
        bot.send_message(msg.chat.id, "❌ No has respondido a ningún mensaje para obtener los datos del usuario").await?;
        return Ok(())
    };

    let data = bot.get_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
    bot.send_message(msg.chat.id, format!("{data:#?}")).await?;

    Ok(())
}
*/