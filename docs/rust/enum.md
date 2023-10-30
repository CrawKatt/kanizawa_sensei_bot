Un Enum es un tipo que almacena diferentes variantes, almacena diferentes opciones\.

Ejemplo en Rust:
```rust
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn main() {
    let color = Color::Red;

    match color {
        Color::Red => println!("El color es rojo"),
        Color::Green => println!("El color es verde"),
        Color::Blue => println!("El color es azul"),
        Color::Yellow => println!("El color es amarillo"),
    }
}
```