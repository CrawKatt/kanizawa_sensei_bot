El tipo de dato `Option<T>` es un tipo de dato que puede ser alguno de dos valores: `Some(T)`\(Hay valor\) o None \(Sin valor\)\.

Ejemplo en Rust:

```rust
fn main() {
    let valor: Option<i32> = Some(5);
    let valor2: Option<i32> = None;
}
```

Consejo: En Rust, no existe el `Null` y se utiliza `None` para indicar que no hay valor además de
ser necesario desestructurar el valor antes de utilizarlo con otros tipos\.

```rust
fn main() {
    let valor: Option<i32> = Some(5);
    let valor2: i32 = 5;
    
    // Esto dará un error porque los tipos de datos no coinciden,
    // no se puede sumar un Option<i32> con un i32
    let suma = valor + valor2;
    println!("El valor de la suma es: {}", suma);
}
```