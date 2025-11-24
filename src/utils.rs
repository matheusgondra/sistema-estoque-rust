use std::io;

pub fn get_input() -> Result<String, io::Error> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

pub enum MenuOption {
    Exit = 0,
    RegisterProduct = 1,
    ListItems = 2,
    SearchItem = 3,
    AddStock = 4,
    RemoveStock = 5,
}

impl MenuOption {
    pub fn from_u8(value: u8) -> Option<MenuOption> {
        match value {
            0 => Some(MenuOption::Exit),
            1 => Some(MenuOption::RegisterProduct),
            2 => Some(MenuOption::ListItems),
            3 => Some(MenuOption::SearchItem),
            4 => Some(MenuOption::AddStock),
            5 => Some(MenuOption::RemoveStock),
            _ => None,
        }
    }
}
