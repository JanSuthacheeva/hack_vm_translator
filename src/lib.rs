use std::error::Error;

mod cleaner;
mod code_writer;
mod parser;

pub fn translate(input: &str, name: &str) -> Result<String, Box<dyn Error>> {
    let program = cleaner::clean_program(input);

    let commands = parser::parse(program)?;

    let output = code_writer::translate(commands, name)?;

    Ok(output)
}

pub fn initialize() -> String {
    String::from("// initialization\n@256\nD=A\n@SP\nM=D\n\n")
}
