use crate::parser::{Arithmetic, Command, PushPop, Segment};
use std::error::Error;

pub fn translate(commands: Vec<Command>, name: &str) -> Result<String, Box<dyn Error>> {
    let mut res = String::from("");
    let mut i: u16 = 0;
    for command in commands {
        let assembly_code = match command {
            Command::Arithmetic(c) => translate_arithmetic(c, &mut i),
            Command::Push(c) => translate_push(c, name),
            Command::Pop(c) => translate_pop(c, name),
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
}
