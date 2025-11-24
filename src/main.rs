mod ui;
mod utils;
mod product;

use ui::terminal;
use product::Product;

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
            1 => register_product(),
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

fn register_product() {
    println!("Digite o nome do produto: ");
    let name = utils::get_input().expect("Falha ao ler o nome do produto");
    if name.is_empty() {
        println!("Nome do produto não pode ser vazio.");
        return;
    }

    println!("Digite a unidade do produto. Ex: m (metro), un (unidade)");
    let unity = utils::get_input().expect("Falha ao ler a unidade do produto");
    if unity.is_empty() {
        println!("Unidade do produto não pode ser vazio.");
        return;
    }

    println!("Digite o endereço do produto no estoque:");
    let address = utils::get_input().expect("Falha ao ler o endereço do produto");
    if address.is_empty() {
        println!("Endereço do produto não pode ser vazio.");
        return;
    }

    let product = Product::new(name, unity, address);

    println!("{}", product.info());
}