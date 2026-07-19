use std::error::Error;

mod cleaner;

pub fn translate(input: &str) -> Result<String, Box<dyn Error>> {

    let program = cleaner::clean_program(input.lines().collect());

    Ok(String::from(""))
}
