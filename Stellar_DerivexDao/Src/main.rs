// src/main.rs
mod lib;
use lib::soma;

fn main() {
    let resultado = soma(2, 3);
    println!("Resultado: {}", resultado);
}