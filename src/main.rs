mod ui;
mod utils;

use ui::terminal;

fn main() {
    loop {
        terminal::show_menu();

        let input = utils::get_input().expect("Falha ao ler a entrada do usuário");
        
        let option: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, insira um número válido.");
                continue;
            }
        };

        match option {
            1 => println!("Opção 1 selecionada: Cadastrar Novo Item"),
            2 => println!("Opção 2 selecionada: Listar Itens em Estoque"),
            3 => println!("Opção 3 selecionada: Buscar Item Existente"),
            4 => println!("Opção 4 selecionada: Dar Entrada no Item"),
            5 => println!("Opção 5 selecionada: Dar Saída no Item"),
            0 => {
                println!("Saindo do programa...");
                break;
            }
            _ => println!("Opção inválida! Por favor, tente novamente."),
        }   
    }
}