use crate::parser::{Arithmetic, Branching, BranchingCommand, Call, Command, Function, PushPop, Segment};
use std::error::Error;

pub fn translate(commands: Vec<Command>, name: &str) -> Result<String, Box<dyn Error>> {
    let mut res = String::from("");
    let mut i: u16 = 0;
    for command in commands {
        let assembly_code = match command {
            Command::Arithmetic(c) => translate_arithmetic(c, &mut i),
            Command::Push(c) => translate_push(c, name),
            Command::Pop(c) => translate_pop(c, name),
            Command::Branching(c) => translate_branching(c),
            Command::Function(c) => translate_function(c, name),
            Command::Return => translate_return(name),
            Command::Call(c) => translate_call(c, name),
        };
        res.push('\n');
        res.push_str(&assembly_code);
    }
    Ok(res)
}

fn translate_arithmetic(command: Arithmetic, i: &mut u16) -> String {
    let operation = match command {
        Arithmetic::Add => "M=D+M",
        Arithmetic::Sub => "M=M-D",
        Arithmetic::And => "M=D&M",
        Arithmetic::Or => "M=D|M",
        Arithmetic::Neg => "M=-M",
        Arithmetic::Eq => "JNE",
        Arithmetic::Gt => "JLE",
        Arithmetic::Lt => "JGE",
        Arithmetic::Not => "M=!M",
    };

    match command {
        Arithmetic::Not | Arithmetic::Neg => {
            format!("// {command}\n@SP\nM=M-1\nA=M\n{operation}\n@SP\nM=M+1\n")
        }
        Arithmetic::Eq | Arithmetic::Gt | Arithmetic::Lt => {
            *i += 1;
            format!(
                "// {command}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@LBL_{i}\nD;{operation}\n@0\nA=M\nM=-1\n@END_LBL_{i}\n0;JMP\n(LBL_{i})\n@0\nA=M\nM=0\n(END_LBL_{i})\n@SP\nM=M+1\n"
            )
        }
        Arithmetic::Add | Arithmetic::Sub | Arithmetic::And | Arithmetic::Or => format!(
            "// {command}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@SP\nM=M-1\nA=M\n{operation}\n@SP\nM=M+1\n"
        ),
    }
}

fn translate_branching(command: Branching) -> String {
    let label = command.label;
    let cmd = command.command;
    match cmd {
        BranchingCommand::Label => format!("// {cmd} {label}\n({label})\n"),
        BranchingCommand::Goto => format!("// {cmd} {label}\n@{label}\n0;JMP\n"),
        BranchingCommand::IfGoto => format!("// {cmd} {label}\n@SP\nM=M-1\nA=M\nD=M\n@{label}\nD;JNE\n"),
    }
}

fn translate_call(command: Call, name: &str) -> String {
    let fn_name = command.name;
    let n_args = command.n_args;
    let mut res = format!("// call {name}.{fn_name} {n_args}\n");
    res.push_str(&format!("//   push returnAddress\n@{name}.{fn_name}_retAddr\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"));
    res.push_str("//   push LCL\n@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    res.push_str("//   push ARG\n@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    res.push_str("//   push THIS\n@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    res.push_str("//   push THAT\n@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    res.push_str(&format!("//   ARG = SP - nArgs\n@{n_args}\nD=A\n@SP\nD=M-D\n@ARG=M=D\n"));
    res.push_str("//   LCL = SP\n@SP\nD=M\n@LCL\nM=D\n");
    res.push_str(&format!("//   label returnAddress\n({name}.{fn_name}_retAddr)\n"));

    res
}

fn translate_function(command: Function, name: &str) -> String {
    let fn_name = command.name;
    let n_vars = command.n_vars;
    let mut res = format!("// function {name}.{fn_name} {n_vars}\n");
    for _n in 0..command.n_vars {
        let pp = PushPop {
            segment: Segment::Constant,
            i: 0,
        };
        res.push_str(&translate_push(pp, name));
    }

    res
}

fn translate_pop(command: PushPop, name: &str) -> String {
    let segment = command.segment;
    let i = command.i;

    let addr = match segment {
        Segment::Temp => (5 + i).to_string(),
        Segment::Static => format!("{name}.{i}"),
        Segment::Pointer => {
            if i == 0 {
                "THIS".to_string()
            } else {
                "THAT".to_string()
            }
        }
        Segment::Local => "LCL".to_string(),
        Segment::Argument => "ARG".to_string(),
        Segment::This => "THIS".to_string(),
        Segment::That => "THAT".to_string(),
        Segment::Constant => unreachable!("Cannot pop a constant"),
    };

    match segment {
        Segment::Local | Segment::Argument | Segment::This | Segment::That => format!(
            "// pop {segment} {i}\n@{i}\nD=A\n@{addr}\nD=D+M\n@R13\nM=D\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@R13\nA=M\nM=D\n"
        ),
        Segment::Temp | Segment::Static | Segment::Pointer => {
            format!("// pop {segment} {i}\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@{addr}\nM=D\n")
        }
        Segment::Constant => unreachable!("Cannot pop a constant"),
    }
}


fn translate_push(command: PushPop, name: &str) -> String {
    let segment = command.segment;
    let i = command.i;

    if segment == Segment::Constant {
        return format!("// push {segment} {i}\n@{i}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
    }

    let addr = match segment {
        Segment::Temp => (5 + i).to_string(),
        Segment::Static => format!("{name}.{i}"),
        Segment::Pointer => {
            if i == 0 {
                "THIS".to_string()
            } else {
                "THAT".to_string()
            }
        }
        Segment::Local => "LCL".to_string(),
        Segment::Argument => "ARG".to_string(),
        Segment::This => "THIS".to_string(),
        Segment::That => "THAT".to_string(),
        Segment::Constant => unreachable!("should have returned already"),
    };

    match segment {
        Segment::Local | Segment::Argument | Segment::This | Segment::That => format!(
            "// push {segment} {i}\n@{i}\nD=A\n@{addr}\nD=D+M\nA=D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        ),
        Segment::Temp | Segment::Static | Segment::Pointer => {
            format!("// push {segment} {i}\n@{addr}\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n")
        }
        Segment::Constant => unreachable!("should have returned already"),
    }
}


fn translate_return(name: &str) -> String {
    let mut res = String::from("// return\n//  endFrame = LCL\n@LCL\nD=M\n@R14\nM=D\n//   retAddr = *(endFrame - 5)\n@5\nD=A\n@R14\nD=M-D\n@R15\nM=D\n");
    let pp = PushPop {
        segment: Segment::Argument,
        i: 0,
    };
    res.push_str(&translate_pop(pp, name));
    res.push_str("//    SP = ARG + 1\n@ARG\nD=M+1\n@SP\nM=D\n");
    res.push_str("//    THAT = *(endFrame - 1)\n@1\nD=A\n@R14\nD=M-D\nA=D\nD=M\n@THAT\nM=D\n");
    res.push_str("//    THIS = *(endFrame - 2)\n@2\nD=A\n@R14\nD=M-D\nA=D\nD=M\n@THIS\nM=D\n");
    res.push_str("//    ARG = *(endFrame - 3)\n@3\nD=A\n@R14\nD=M-D\nA=D\nD=M\n@ARG\nM=D\n");
    res.push_str("//    LCL = *(endFrame - 4)\n@4\nD=A\n@R14\nD=M-D\nA=D\nD=M\n@LCL\nM=D\n");
    res.push_str("//    goto retAddr\n@R15\nA=M\n0;JMP\n");

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_pop_emits_short_command() {
        let input = PushPop {
            segment: Segment::Temp,
            i: 2,
        };
        assert_eq!(
            translate_pop(input, ""),
            "// pop temp 2\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@7\nM=D\n"
        );
    }

    #[test]
    fn translate_pop_emits_long_command() {
        let input = PushPop {
            segment: Segment::Local,
            i: 2,
        };
        assert_eq!(
            translate_pop(input, ""),
            "// pop local 2\n@2\nD=A\n@LCL\nD=D+M\n@R13\nM=D\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@R13\nA=M\nM=D\n"
        );
    }

    #[test]
    fn translate_push_emits_long_command() {
        let input = PushPop {
            segment: Segment::Local,
            i: 2,
        };
        assert_eq!(
            translate_push(input, ""),
            "// push local 2\n@2\nD=A\n@LCL\nD=D+M\nA=D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn translate_push_emits_short_command() {
        let input = PushPop {
            segment: Segment::Static,
            i: 2,
        };
        assert_eq!(
            translate_push(input, "lol"),
            "// push static 2\n@lol.2\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn translate_arithmetic_emits_short_command() {
        let input = Arithmetic::Not;
        let mut i = 1;
        assert_eq!(
            translate_arithmetic(input, &mut i),
            "// not\n@SP\nM=M-1\nA=M\nM=!M\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn translate_arithmetic_emits_long_command() {
        let input = Arithmetic::Eq;
        let mut i = 1;
        assert_eq!(
            translate_arithmetic(input, &mut i),
            "// eq\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@SP\nM=M-1\nA=M\nD=M-D\n@LBL_2\nD;JNE\n@0\nA=M\nM=-1\n@END_LBL_2\n0;JMP\n(LBL_2)\n@0\nA=M\nM=0\n(END_LBL_2)\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn translate_function_emits_command() {
        let input = Function {
            name: String::from("test"),
            n_vars: 3
        };
        let name = "testName";
        assert_eq!(
            translate_function(input, &name),
            "// function testName.test 3\n// push constant 0\n@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n// push constant 0\n@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n// push constant 0\n@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"

        );
    }

    #[test]
    fn translates_chained_commands() {
        let input = vec![
            Command::Push(PushPop {
                segment: Segment::Constant,
                i: 2,
            }),
            Command::Arithmetic(Arithmetic::Add),
        ];
        assert_eq!(
            translate(input, "").unwrap(),
            "\n// push constant 2\n@2\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n\n// add\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@SP\nM=M-1\nA=M\nM=D+M\n@SP\nM=M+1\n"
        );
    }

    #[test]
    fn translate_branching_emits_label_command() {
        let input = Branching {
            command: BranchingCommand::Label,
            label: String::from("testLabel")
        };
        assert_eq!(
            translate_branching(input),
            "// label testLabel\n(testLabel)\n"
        );
    }

    #[test]
    fn translate_branching_emits_goto_command() {
        let input = Branching {
            command: BranchingCommand::Goto,
            label: String::from("testLabel")
        };
        assert_eq!(
            translate_branching(input),
            "// goto testLabel\n@testLabel\n0;JMP\n"
        );
    }

    #[test]
    fn translate_branching_emits_if_goto_command() {
        let input = Branching {
            command: BranchingCommand::IfGoto,
            label: String::from("testLabel")
        };
        assert_eq!(
            translate_branching(input),
            "// if-goto testLabel\n@SP\nM=M-1\nA=M\nD=M\n@testLabel\nD;JNE\n"
        );
    }

    #[test]
    fn translate_return_emits_proper_command() {
        let input = "testFile";
        assert_eq!(
            translate_return(input),
            "// return\n//  endFrame = LCL\n@LCL\nD=M\n@R14\nM=D\n//   retAddr = *(endFrame - 5)\n@5\nD=A\n@R14\nD=M-D\n@R15\nM=D\n// pop argument 0\n@0\nD=A\n@ARG\nD=D+M\n@R13\nM=D\n@SP\nM=M-1\nD=M\nA=D\nD=M\n@R13\nA=M\nM=D\n//    SP = ARG + 1\n@ARG\nD=M+1\n@SP\nM=D\n//    THAT = *(endFrame - 1)\n@1\nD=A\n@R14\nD=M-D\nA=D\nD=M\n@THAT\nM=D\n//    THIS = *(endFrame - 2)\n@2\nD=A\n@R14\nD=M-D\nA=D\nD=M\n@THIS\nM=D\n//    ARG = *(endFrame - 3)\n@3\nD=A\n@R14\nD=M-D\nA=D\nD=M\n@ARG\nM=D\n//    LCL = *(endFrame - 4)\n@4\nD=A\n@R14\nD=M-D\nA=D\nD=M\n@LCL\nM=D\n//    goto retAddr\n@R15\nA=M\n0;JMP\n"
        );
    }
}
