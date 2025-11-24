use std::io;

pub fn get_input() -> Result<String, io::Error> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}