#![allow(non_snake_case)]
use std::io::{self, Write};
mod f;
use f::*;

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    print!("Digite A Posicao X: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut x).expect("Erro Na Entrada De X");
    print!("Digite A Posicao Y: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut y).expect("Erro Na Entrada De Y");

    //Converte as strings x e y para o formato de double, ou duplo ponto flutuante
    let x:f64 = x.trim().parse().expect("Erro Na Conversão De X");
    let y:f64 = y.trim().parse().expect("Erro Na Conversão De Y");


  
}
