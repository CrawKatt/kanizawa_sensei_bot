*let else* se utiliza para desempaquetar y hacer coincidir un valor de un tipo Option\<T\> o Result de forma concisa\.
a diferencia de *if let*, *let else* es más semántico al aplicar una cláusula de guarda, ahorrando las anidaciones
de *if let*\.

Ejemplo en Rust:
```rust
fn main() {
    let option_value: Option<i32> = Some(5);

    let Some(x) = option_value else {
        println!("La opción no contiene un valor.");
        return
    };
    println!("El valor es: {}", x);
}
```
Consejo: *let else* funciona de forma opuesta a *if let* comenzamos negando la condición y si la condición
*else* no se cumple, ignoramos el bloque de código dentro de *else*