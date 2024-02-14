use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Advinhe um número!");

    let numero_secreto = rand::thread_rng().gen_range(1..=1000);

    loop {
        println!("Digite o seu chute.");

        let mut chute = String::new();

        io::stdin()
            .read_line(&mut chute)
            .expect("Erro ao ler a linha.");

        let chute: u32 = match chute.trim().parse() {
            Ok(numero) => numero,
            Err(_) => continue,
        };

        println!("Seu chute: {chute}");

        match chute.cmp(&numero_secreto) {
            Ordering::Less => println!("Menor que o número secreto!"),
            Ordering::Greater => println!("Maior que o número secreto!"),
            Ordering::Equal => {
                println!("Você ganhou!!!!!!!");
                break;
            }
        }
    }
}
