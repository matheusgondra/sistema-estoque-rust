mod ui;

use std::io;
use ui::terminal;

fn main() {
    loop {
        terminal::show_menu();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
        
        let option: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, insira um número válido.");
                continue;
            }
        };

        match option {
            1 => println!("Opção 1 selecionada: Cadastrar Novo Item"),
            0 => {
                println!("Saindo do programa...");
                break;
            }
            _ => println!("Opção inválida! Por favor, tente novamente."),
        }   
    }
}