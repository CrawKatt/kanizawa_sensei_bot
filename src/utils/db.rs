use std::fs::File;
use std::io;
use std::io::{Read, Write};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use teloxide_core::types::Message;
pub use surrealdb::Result as SurrealResult;
use crate::utils::backup_data;

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Data {
    first_name: String,
    last_name: String, // Es Option<String> pero se requiere tipo String para evitar guardar Some("last_name") en la base de datos
    user_id: String, // String Necesario para evitar Overflow
    username: String, // Es Option<String> pero se requiere tipo String para evitar guardar Some("username") en la base de datos
}

pub async fn get_user_data(msg: Message) -> SurrealResult<()> {
    // Conectar a la base de datos
    DB.use_ns("teloxide-namespace").use_db("teloxide").await?;

    // Se usa `.clone()` ya que `.to_string()` aplica `.to_owned()` que a su vez aplica `.clone()` internamente
    let first_name = msg.from().unwrap().first_name.clone();
    let last_name = msg.from().unwrap().last_name.as_ref().unwrap_or(&String::from("Ninguno")).clone();
    let user_id = msg.from().unwrap().id.to_string();
    let username = msg.from().unwrap().username.as_ref().unwrap_or(&String::from("Ninguno")).clone();

    // Obtener los datos del usuario que envió el mensaje
    let data = Data {
        first_name,
        last_name,
        user_id: user_id.clone(), // El user_id debe ser almacenado como String para evitar Overflow
        username,
    };

    // NO USAR FORMAT EN LAS QUERYS, EL MÉTODO BIND SE ENCARGA DE TOMAR VARIABLES A TRAVÉS DE `$`
    // Obtener los datos del usuario de la Base de Datos para evitar duplicados
    let sql_query = "SELECT * FROM users WHERE user_id = $user_id";
    let database_info: Option<Data> = DB
        .query(sql_query)
        .bind(("user_id", user_id)) // pasar el valor
        .await?
        .take(0)?; // take(0) requiere un Option<T> para funcionar

    let Some(database_user) = database_info else {
        let _created: Vec<Data> = DB.create("users").content(data).await?;
        backup_data().await?;
        return Ok(())
    };

    // Si el usuario ya existe en la base de datos, no hacer nada
    if database_user == data {
        return Ok(())
    }

    // Si el usuario existe pero sus datos no coinciden con los de la base de datos, actualizarlos
    if database_user.username != data.username || database_user.first_name != data.first_name || database_user.last_name != data.last_name {
        update_data(data.first_name, data.last_name, data.user_id, data.username).await?;
    }

    Ok(())
}

async fn update_data(
    first_name: String,
    last_name: String,
    user_id: String,
    username: String
) -> SurrealResult<()> {

    // NO USAR FORMAT EN LAS QUERYS, EL MÉTODO BIND SE ENCARGA DE TOMAR VARIABLES A TRAVÉS DE `$`
    let sql_query = "UPDATE users SET first_name = $first_name, last_name = $last_name, username = $username WHERE user_id = $user_id";

    // Cuando se actualiza o se requiere más de un solo dato, utilizar múltiples binds para cada dato
    DB.query(sql_query)
        .bind(("first_name", first_name.clone()))
        .bind(("last_name", last_name.clone()))
        .bind(("username", username.clone()))
        .bind(("user_id", user_id.clone()))
        .await?;

    update_backup(first_name, last_name, username, user_id).await?;

    Ok(())
}

async fn update_backup(
    first_name: String,
    last_name: String,
    username: String,
    user_id: String
) -> SurrealResult<()> {

    // Seleccionar todos los datos de la tabla "users"
    let data: Vec<Data> = DB.select("users").await?;

    // Leer el contenido del archivo backup.json
    let mut file = File::open("backup.json").expect("❌ No se pudo abrir el archivo de respaldo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("❌ No se pudo leer el archivo de respaldo");

    // Analizar los datos del archivo en una variable Vec<Data>
    let mut backup_data: Vec<Data> = serde_json::from_str(&contents).expect("❌ No se pudo analizar el archivo de respaldo");

    // Buscar el usuario correspondiente en Vec<Data> y actualizar los campos
    for user in &mut backup_data {
        if user.user_id == user_id {
            user.username = username;
            user.first_name = first_name;
            user.last_name = last_name;
            break;
        }
    }

    // Serializar los datos actualizados en formato JSON
    let serialized_data = serde_json::to_string_pretty(&data).expect("❌ No se pudo serializar los datos");

    // Sobrescribir solo los datos del usuario correspondiente en el archivo backup.json
    let mut file = File::create("backup.json").expect("❌ No se pudo crear el archivo de respaldo");
    file.write_all(serialized_data.as_bytes()).expect("❌ No se pudo escribir en el archivo de respaldo");

    Ok(())
}

// Todo: armar custom error para cuando no le pases un argumento valido al comando, si estas 100%
// seguro que el comando `/ban {algo}` siempre va a tener {algo} no esta mal usar unwrap, a eso se
// lo considera safety si la API te lo asegura
pub async fn send_data(msg: Message) -> SurrealResult<u64> {
    let Some(text) = msg.text() else { return Ok(0) };
    let (query, column, username) = resolve_indirection(text);
    let user: Option<Data> = DB
        .query(query) // pasar la query
        .bind((column, username)) // pasar el valor
        .await?
        .take(0)?; // take(0) requiere un Option<T> para funcionar
    let user = user.unwrap_or_default(); // variable user para parsear el user_id
    let user_id = user.user_id.parse::<u64>().unwrap_or_default(); // parsear el user_id a u64

    Ok(user_id)
}

/// user: Usuario del /ban {algo} si sanitizar
/// Retorna una tupla siendo el primer valor la query, el segundo es la columna
/// `username | first_name` y el tercero el usuario
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

pub fn delete_user_backup(user_id: String) -> Result<(), io::Error> {
    // Leer el contenido del archivo backup.json
    let mut file = File::open("backup.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Analizar los datos del archivo en una variable Vec<Data>
    let mut backup_data: Vec<Data> = serde_json::from_str(&contents)?;

    // Buscar el usuario correspondiente en Vec<Data> y eliminarlo
    backup_data.retain(|user| user.user_id != user_id);

    // Serializar los datos actualizados en formato JSON
    let serialized_data = serde_json::to_string_pretty(&backup_data)?;

    // Sobrescribir los datos actualizados en el archivo backup.json
    let mut file = File::create("backup.json")?;
    file.write_all(serialized_data.as_bytes())?;

    Ok(())

}