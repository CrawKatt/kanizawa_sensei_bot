pub mod db;

use std::fs::File;
use std::io::{BufReader, Write};
use std::time::Duration;
use tokio::time::sleep;
use teloxide_core::prelude::{ChatId, Requester};
use teloxide_core::types::{Message, MessageId, ChatMember, ChatMemberStatus, User};

use crate::prelude::Bot;
use crate::utils::db::{Data, DB, send_data, SurrealResult};

trait UnwrapUserData {
    fn unwrap_data(&self) -> String;
}

impl UnwrapUserData for Option<String> {
    fn unwrap_data(&self) -> String {
        self.as_ref().map_or_else(|| String::from("Ninguno"), std::clone::Clone::clone)
    }
}

/// This Trait is used to check if a user is an Admin or Owner
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

/// This Trait is used to delete a message after
/// a certain time specified by the method
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

/// This Trait is used to extract the `user_id` from a message
/// and to extract the `user_id` from a reply to a message
/// or from a Join Event
pub trait MessageExt {
    async fn parse_id(&self) -> u64;
    fn extract_new_member_info<'user>(&'user self, msg: &'user Message) -> Option<&User>;
}

impl MessageExt for Message {
    /// # Parse the message to get the `user_id`
    /// Parse the message to get the `user_id` from:
    /// - Reply to a Message
    /// - Send a Message with @username or `user_id` as an argument **(e.g. /ban @username, /ban 12345678)**
    ///
    /// If the `user_id` is not found, return 404 from `unwrap_or_else` and print an error message
    /// in the console
    async fn parse_id(&self) -> u64 {
        let Some(replied) = self.reply_to_message() else {

            // Get the user_id from the Database by @username
            let username_u64 = send_data(self.clone()).await.unwrap_or_else(|e| {
                println!("Error al enviar el user_id desde la base de datos: {e:#?}");
                404
            });

            // First get a user_id from a message if not found a user_id, get the user_id from the @username
            return self.text()
                .and_then(|text| text.split_once(' '))
                .map(|(_, a)| a.trim())
                .and_then(|trimmed| trimmed.parse::<u64>().ok())
                .unwrap_or(username_u64); // If a user_id is not found, return the user_id associated with the @username.
        };
        replied.text().unwrap_or_default().parse::<u64>().unwrap_or(404)
    }

    /// # Extract the `user_id` from a Join Event
    /// Extract the `user_id` from a Join Event to use it in multiple commands
    /// Lifetimes are used to avoid unnecessary copying
    fn extract_new_member_info<'user>(&'user self, msg: &'user Message) -> Option<&User> {
        msg.reply_to_message()?.new_chat_members()?.first()
    }
}

/// This function is used to save the data in a JSON file.
/// (The data in `SurrealDB` is not persistent in Memory)
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

/// This function is used to load data from a JSON file when starting the Bot.
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
        DB.create::<Vec<Data>>("users").content(item).await?;
    }

    Ok(())
}