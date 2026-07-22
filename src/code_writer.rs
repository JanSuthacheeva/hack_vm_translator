
use std::error::Error;
use crate::parser::{Command, Arithmetic, PushPop, Segment};


pub fn translate(commands: Vec<Command>, name: &str) -> Result<String, Box<dyn Error>> {

    let mut res = String::from("");
    let mut i: u16 = 0;
    for command in commands {
        let assembly_code = match command {
            Command::Arithmetic(c) =>translate_arithmetic(c, &mut i),
            Command::Push(c) => translate_push(c, name),
            Command::Pop(c) => translate_pop(c, name),
            _ => String::from("")
        };
        res.push_str("\n");
        res.push_str(&assembly_code);
    }
    Ok(res)
}

fn translate_arithmetic(command: Arithmetic, i: &mut u16) -> String {

    let mut comment = "";
    let mut operation = "";

    match command.instruction.as_str() {
        "add" => {
            comment = "add";
            operation = "M=D+M";
        },
        "sub" => {
            comment = "sub";
            operation = "M=M-D";
        },
        "and" => {
            comment = "and";
            operation = "M=D&M";
        },
        "or" => {
            comment = "or";
            operation = "M=D|M";
        },
        "neg" => {
            comment = "neg";
            operation = "M=-M";
        },
        "eq" => {
            comment = "eq";
            operation = "JNE";
        },
        "gt" => {
            comment = "gt";
            operation = "JLE";
        },
        "lt" => {
            comment = "lt";
            operation = "JGE";
        },
        "not" => {
            comment = "not";
            operation = "M=!M";
        }

        _ => (),
        }
    if ["neg", "not"].contains(&comment) {
        return format!("// {comment}\n@SP\nM=M-1\nA=M\n{operation}\n@SP\nM=M+1\n");
    }
    if ["eq", "gt", "lt"].contains(&comment) {
        *i = *i + 1;
        return format!("// {comment}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@LBL_{i}\nD;{operation}\n@0\nA=M\nM=-1\n@END_LBL_{i}\n0;JMP\n(LBL_{i})\n@0\nA=M\nM=0\n(END_LBL_{i})\n@SP\nM=M+1\n");
    }

    format!("// {comment}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@SP\nM=M-1\nA=M\n{operation}\n@SP\nM=M+1\n")
}

fn translate_push(command: PushPop, name: &str) -> String {
    
    let segment = command.segment;
    let i = command.i;
    let mut addr = String::from("");
    let mut loc = String::from("");


    match segment {
        Segment::Constant => {
            return format!("// push {segment} {i}\n@{i}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        },
        Segment::Temp => {
            addr = (5 + i).to_string();
        }
        Segment::Static => {
            addr = format!("{name}.{i}");
        },
        Segment::Pointer => {
            addr = if i == 0 { "THIS".to_string() } else { "THAT".to_string() };
        },
        Segment::Local => {
            loc = "LCL".to_string();
        },
        Segment::Argument => {
            loc = "ARG".to_string();
        }
        Segment::This => {
            loc = "THIS".to_string();
        },
        Segment::That => {
            loc = "THAT".to_string();
        },
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


    match segment {
        Segment::Temp => {
            addr = (5 + i).to_string();
        }
        Segment::Static => {
            addr = format!("{name}.{i}");
        },
        Segment::Pointer => {
            addr = if i == 0 { "THIS".to_string() } else { "THAT".to_string() };
        },
        Segment::Local => {
            loc = "LCL".to_string();
        },
        Segment::Argument => {
            loc = "ARG".to_string();
        }
        Segment::This => {
            loc = "THIS".to_string();
        },
        Segment::That => {
            loc = "THAT".to_string();
        },
        _ => ()
    };
    if loc != "" {
        return format!("// pop {segment} {i}\n@{i}\nD=A\n@{loc}\nD=D+M\n@var\nM=D\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@var\nA=M\nM=D\n");
    }

    format!("// pop {segment} {i}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@{addr}\nM=D\n")
}
