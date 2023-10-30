Los generics nos permiten crear tipos de datos genéricos, que pueden ser de cualquier tipo\.

Ejemplo en Rust:
```rust
struct Generic<T> {
    valor: T,
}

fn main() {
    let entero = Generic {valor: 5,};
    let flotante = Generic {valor: 5.0,};

    println!("El valor entero es: {:?}", entero.valor);
    println!("El valor flotante es: {:?}", flotante.valor);
}
```