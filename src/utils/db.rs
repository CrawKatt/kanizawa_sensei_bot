use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use teloxide_core::types::Message;
use surrealdb::Result as SurrealResult;

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Data {
    first_name: String,
    last_name: String, // Es Option<String> pero se requiere tipo String para evitar guardar Some("last_name") en la base de datos
    user_id: String, // String Necesario para evitar Overflow
    username: String, // Es Option<String> pero se requiere tipo String para evitar guardar Some("username") en la base de datos
}

pub async fn get_user_data(msg: Message) -> SurrealResult<()> {
    DB.use_ns("teloxide-namespace").use_db("teloxide").await?;

    let none = &String::from("Ninguno");

    let first_name = msg.from().unwrap().first_name.to_string();
    let last_name = msg.from().unwrap().last_name.as_ref().unwrap_or(none).to_string();
    let user_id = msg.from().unwrap().id.to_string();
    let username = msg.from().unwrap().username.as_ref().unwrap_or(none).to_string();

    // Obtener los datos del usuario que envió el mensaje
    let data = Data {
        first_name,
        last_name,
        user_id, // El user_id debe ser almacenado como String para evitar Overflow
        username,
    };

    let created: Vec<Data> = DB.create("users").content(data).await?;
    println!("{created:#?}");

    Ok(())
}

// Todo: armar custom error para cuando no le pases un argumento valido al comando, si estas 100%
// todo: crear un update para actualizar la Base de Datos cuando se repitan los datos del usuario
// seguro que el comando `/ban {algo}` siempre va a tener {algo} no esta mal usar unwrap, a eso se
// lo considera safety si la API te lo asegura
pub async fn send_data(msg: Message) -> SurrealResult<u64> {
    let Some(text) = msg.text() else { return Ok(0) };
    let (query, column, username) = resolve_indirection(text);
    let user: Option<Data> = DB
        .query(query)
        .bind((column, username)) // pasar el valor
        .await?
        .take(0)?; // take(0) requiere un Option<T> para funcionar
    let user = user.unwrap_or_default();
    let user_id = user.user_id.parse::<u64>().unwrap_or_default();

    Ok(user_id)
}

/// user: Usuario del /ban {algo} si sanitizar
/// Retorna una tupla siendo el primer valor la query, el segundo es la columnda `username |
/// first_name` y el tercero el usuario
pub fn resolve_indirection(user: &str) -> (String, String, String) {
    let separator = if user.contains('@') { '@' } else { ' ' };
    let column = match separator {
        '@' => "username",
        _ => "first_name",
    };

    let username = user
        .split_once(separator)
        .map(|x| x.1)
        .unwrap_or_default()
        .to_owned();

    let query = format!("SELECT * FROM users WHERE {column} = ${column}");
    (query, column.to_owned(), username)
}