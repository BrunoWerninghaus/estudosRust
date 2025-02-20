use std::io;

fn main() {

    let soma : Vec<String> = Vec::new();
    let sub : Vec<String> = Vec::new();
    let div : Vec<String> = Vec::new();
    let mult : Vec<String> = Vec::new();

    let mut opc = String::new();

    println!("\nInsira os números: \n");

    io::stdin().read_line(&mut opc).expect("Msg");

    let opc:i8=opc.trim().parse().expect("Erro // L16");

    println!("//CALCULADORA\\");
}
