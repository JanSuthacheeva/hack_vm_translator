
use std::error::Error;
use crate::parser::{Command, Arithmetic, PushPop};


pub fn translate(commands: Vec<Command>, name: &str) -> Result<String, Box<dyn Error>> {

    let mut res = String::from("");
    let mut assembly_code = String::from("");
    for command in commands {
        match command {
            Command::Arithmetic(c) => {
                assembly_code = translate_arithmetic(c);
            }
            Command::Push(c) => {
                assembly_code = translate_push(c, name);
            },
            Command::Pop(c) => {
                assembly_code = translate_pop(c, name);
            },
            _ => ()
        };
        res = format!("{res}\n{assembly_code}");
    }
    Ok(res)
}

fn translate_arithmetic(command: Arithmetic) -> String {

    let mut comment = "";
    let mut operation = "";

    match command.instruction.as_str() {
        "add" => {
            comment = "add";
            operation = "M=M+D";
        },
        "sub" => {
            comment = "sub";
            operation = "M=M-D";
        },
        _ => (),
        }

    format!("// {comment}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@SP\n{operation}\n@SP\nM=M+1")
}

fn translate_push(command: PushPop, name: &str) -> String {
    
    let segment = command.segment;
    let i = command.i;
    let mut addr = String::from("");
    let mut loc = String::from("");


    match segment.as_str() {
        "constant" => {
            return format!("// push {segment} {i}\n@{i}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        },
        "temp" => {
            addr = (5 + i).to_string();
        }
        "static" => {
            addr = format!("{name}.{i}");
        },
        "pointer" => {
            addr = if i == 0 { "THIS".to_string() } else { "THAT".to_string() };
        },
        "local" => {
            loc = "LCL".to_string();
        },
        "argument" => {
            loc = "ARG".to_string();
        }
        "this" => {
            loc = "THIS".to_string();
        },
        "that" => {
            loc = "THAT".to_string();
        },
        _ => (),
    };
    if loc != String::from("") {
        return format!("// push {segment} {i}\n@{i}\nD=A\n@{loc}\nD=D+M\nA=D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    }

    format!("// push {segment} {i}\n@{addr}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n")
}

fn translate_pop(command: PushPop, name: &str) -> String {
    let mut addr = String::from("");
    let mut loc = String::from("");
    let segment = command.segment;
    let i = command.i;


    match segment.as_str() {
        "temp" => {
            addr = (5 + i).to_string();
        }
        "static" => {
            addr = format!("{name}.{i}");
        },
        "pointer" => {
            addr = if i == 0 { "THIS".to_string() } else { "THAT".to_string() };
        },
        "local" => {
            loc = "LCL".to_string();
        },
        "argument" => {
            loc = "ARG".to_string();
        }
        "this" => {
            loc = "THIS".to_string();
        },
        "that" => {
            loc = "THAT".to_string();
        },
        _ => (),
    };
    if loc != "" {
        return format!("// pop {segment} {i}\n@{i}\nD=A\n@{loc}\nD=D+M\n@var\nM=D\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@var\nA=M\nM=D\n");
    }

    format!("// pop {segment} {i}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@{addr}\nM=D\n")
}
