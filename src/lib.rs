use std::error::Error;

mod cleaner;
mod parser;

pub fn translate(input: &str) -> Result<String, Box<dyn Error>> {

    let program = cleaner::clean_program(input.lines().collect());

    let commands = parser::parse(program)?;

    println!("{commands:?}");

    Ok(String::from(""))
}
