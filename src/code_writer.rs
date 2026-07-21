
use std::error::Error;
use crate::parser::{Command, Arithmetic, PushPop};


pub fn translate(commands: Vec<Command>, name: &str) -> Result<String, Box<dyn Error>> {

    let mut res = String::from("");
    let mut assembly_code = String::from("");
    let mut i: u16 = 1;
    for command in commands {
        match command {
            Command::Arithmetic(c) => {
                assembly_code = translate_arithmetic(c, &mut i);
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

fn translate_arithmetic(command: Arithmetic, i: &mut u16) -> String {

    let mut comment = "";
    let mut operation = String::from("");

    match command.instruction.as_str() {
        "add" => {
            comment = "add";
            operation = String::from("M=D+M");
        },
        "sub" => {
            comment = "sub";
            operation = String::from("M=M-D");
        },
        "and" => {
            comment = "and";
            operation = String::from("M=D&M");
        },
        "or" => {
            comment = "or";
            operation = String::from("M=D|M");
        },
        "neg" => {
            comment = "neg";
            operation = String::from("M=-M");
        },
        "eq" => {
            comment = "eq";
            operation = format!("D=M-D\n@EQ_{i}\nD;JNE\n@0\nA=M\nM=-1\n@END_EQ_{i}\n0;JMP\n(EQ_{i})\n@0\nA=M\nM=0\n(END_EQ_{i})");
            *i = *i + 1;
        },
        "gt" => {
            comment = "gt";
            operation = format!("D=M-D\n@GT_{i}\nD;JLE\n@0\nA=M\nM=-1\n@END_GT_{i}\n0;JMP\n(GT_{i})\n@0\nA=M\nM=0\n(END_GT_{i})");
            *i = *i + 1;
        },
        "lt" => {
            comment = "lt";
            operation = format!("D=M-D\n@LT_{i}\nD;JGE\n@0\nA=M\nM=-1\n@END_LT_{i}\n0;JMP\n(LT_{i})\n@0\nA=M\nM=0\n(END_LT_{i})");
            *i = *i + 1;
        },
        "not" => {
            comment = "not";
            operation = String::from("M=!M");
        }

        _ => (),
        }
    if ["neg", "not"].contains(&comment) {
        return format!("// {comment}\n@SP\nM=M-1\nA=M\n{operation}\n@SP\nM=M+1");
    }

    format!("// {comment}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@SP\nM=M-1\nA=M\n{operation}\n@SP\nM=M+1")
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
