Las funciones asincrónicas nos permiten ejecutar código de forma asíncrona\.

Ejemplo en Rust: 
```rust
async fn saluda() {
    println!("¡Hola, mundo!");
}

#[tokio::main]
async fn main() {
    saluda().await;
}
```
Consejo: En Rust, la funcion `main` no puede ser asincrona\.
Para poder ejecutar código asincrónico en Rust, 
debemos usar la macro `#[tokio::main]` de la libreria
`tokio` en la función `main`\.