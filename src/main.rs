use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe o numero!");
    let num_secreto: u8 = rand::thread_rng().gen_range(1..=100);
    println!("{num_secreto}");

    loop {
        println!("Por favor, digite seu palpite...");
        let mut palpite: String = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha na leitura da linha");

        let palpite: u8 = {
            match palpite.trim().parse::<u8>() {
                Ok(value) => value,
                Err(_) => {
                    println!("Digite um numero valido! \n");
                    continue;
                }
            }
        };

        match palpite.cmp(&num_secreto) {
            Ordering::Equal => {
                println!("O senhor advinhou: {palpite}");
                break;
            }

            Ordering::Greater => {
                println!("Ta muito alto... \n");
            }

            Ordering::Less => {
                println!("Ta muito baixo... \n");
            }
        }

        //
    }
}
