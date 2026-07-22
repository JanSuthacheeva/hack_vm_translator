use std::error::Error;

mod cleaner;
mod code_writer;
mod parser;

pub fn translate(input: &str, name: &str) -> Result<String, Box<dyn Error>> {
    let program = cleaner::clean_program(input.lines().collect());

    let commands = parser::parse(program)?;

    let output = code_writer::translate(commands, name)?;

    //println!("{output}");

    Ok(output)
}
