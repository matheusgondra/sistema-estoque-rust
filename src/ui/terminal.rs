use std::io::{self, Write};

pub fn show_menu() {
    println!("*******************************");
    println!("*        MENU PRINCIPAL       *");
    println!("*******************************");
    println!("* 1. Cadastrar Novo Item      *");
    println!("* 2. Exibir Itens             *");
    println!("* 3. Buscar Item              *");
    println!("* 4. Entrada de itens         *");
    println!("* 5. Baixa de itens           *");
    println!("* 0. Sair                     *");
    println!("*******************************");
    print!("Escolha uma opção: ");

    io::stdout().flush().unwrap();
}