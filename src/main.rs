#![allow(non_snake_case)]
#![allow(unused)]
use std::io;
mod f;
use f::*;

fn main(){
    let mut robo_x = String::new();
    let mut robo_y = String::new();

    println!("Insira A Posição X :");
    io::stdin().read_line(&mut robo_x);
    println!("Insira A Posição Y :");
    io::stdin().read_line(&mut robo_y);

    let mut robo_x= robo_x.trim().parse::<f64>().expect("Erro Na Convercao de X");
    let mut robo_y= robo_y.trim().parse::<f64>().expect("Erro Na Convercao de Y");

    calculo_total(&mut robo_x,&mut robo_y);    
    

}