mod ui;
mod utils;
mod product;

use ui::terminal;
use product::Product;
use utils::MenuOption;

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

        match MenuOption::from_u8(option) {
            Some(MenuOption::RegisterProduct) => register_product(),
            Some(MenuOption::ListItems) => println!("Opção 2 selecionada: Listar Itens em Estoque"),
            Some(MenuOption::SearchItem) => println!("Opção 3 selecionada: Buscar Item Existente"),
            Some(MenuOption::AddStock) => println!("Opção 4 selecionada: Dar Entrada no Item"),
            Some(MenuOption::RemoveStock) => println!("Opção 5 selecionada: Dar Saída no Item"),
            Some(MenuOption::Exit) => {
                println!("Saindo do programa...");
                break;
            },
            None => println!("Opção inválida. Por favor, tente novamente.")
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