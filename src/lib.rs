use std::error::Error;

mod cleaner;
mod code_writer;
mod parser;

use crate::parser::{Call, Command};

pub fn translate(input: &str, name: &str) -> Result<String, Box<dyn Error>> {
    let program = cleaner::clean_program(input);

    let commands = parser::parse(program)?;

    let output = code_writer::translate(commands, name)?;

    Ok(output)
}

pub fn initialize() -> String {
    let mut res = String::from("// initialization\n@256\nD=A\n@SP\nM=D\n\n");

    let cmd: Vec<Command> = vec![Command::Call(Call{
        name: String::from("Sys.init"),
        n_args: 0,
    })];

    let output = code_writer::translate(cmd, "").expect("Could not initialize translator");

    
    res.push_str(&output);
    res
}
