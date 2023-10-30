*Variables:* Son un espacio en memoria cuyo valor puede asignarse y cambiar

Ejemplo en Rust:
```rust
fn main() {
    let mi_variable = "valor";
    println!("El valor de mi variable es: {}", mi_variable);
}
```
Consejo: En Rust, las variables *por defecto son inmutables*, eso significa que no puedes cambiar su valor sin definirlo explícitamente\.
para cambiar el valor de una variable en Rust utilizamos la palabra reservada *mut*
```rust
fn main() {
    let mut mi_variable = "hola";
    println!("{}", mi_variable);
    
    mi_variable = "mundo";
    println!("{}", mi_variable);
}
```