Las macros nos permiten escribir código que produce código\.

Ejemplo en Rust:
```rust
macro_rules! say_hello {
    () => (
        println!("Hola");
    );
}

fn main() {
    say_hello!();
}
```
Consejo: Existen las macros declarativas y las macros procedurales\. Las macros declarativas son
más simples mientras que las macros procedurales son más complejas pero poderosas\.
```rust
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
```
```rust
extern crate proc_macro_examples;
use proc_macro_examples::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}
```
