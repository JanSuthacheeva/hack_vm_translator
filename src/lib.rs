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

pub fn initialize() -> Result<String, Box<dyn Error>> {
    // [ ] TODO: check if this is correct
    let init = String::from("// initialization\n@256\nD=A\n@SP\nM=D\n@300\nD=A\n@LCL\nM=D\n@400\nD=A\n@ARG\nM=D\n@3000\nD=A\n@THIS\nM=D\n@3010\nD=A\n@THAT\nM=D\n\n");

    Ok(init)
}
