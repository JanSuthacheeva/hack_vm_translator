use std::error::Error;

#[derive(PartialEq, Debug)]
pub enum Command {
    Arithmetic(Arithmetic),
    Push(PushPop),
    Pop(PushPop),
    Goto,
    If,
    Function,
    Return,
    Call
}

#[derive(PartialEq, Debug)]
pub struct PushPop {
    pub segment: String,
    pub i: u16,
}

#[derive(PartialEq, Debug)]
pub struct Arithmetic {
    pub instruction: String,
}

pub fn parse(program: Vec<&str>) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut result: Vec<Command> = vec![];
    for line in program {
        let command = parse_line(line)?;
        result.push(command);
    }

    Ok(result)
}

fn parse_line(line: &str) -> Result<Command, Box<dyn Error>> {

    let elements = line.split_whitespace();

    match elements.clone().count() {
        1 => handle_arithmetic_command(line),
        3 => {
            handle_memory_command(elements.collect())
        },
        _ => Err("Invalid command: {line}".into()),
    }
}

fn handle_arithmetic_command(line: &str) -> Result<Command, Box<dyn Error>> {
    let valid_commands = ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];
    if !valid_commands.contains(&line) {
        return Err("Invalid command: {line}".into());
    }

    Ok(Command::Arithmetic(
            Arithmetic {
                instruction: String::from(line)
            }
            )
        )

}

fn handle_memory_command(elements: Vec<&str>) -> Result<Command, Box<dyn Error>> {
    let segments = ["local", "argument", "this", "that", "constant", "static", "temp", "pointer"];
    if !segments.contains(&elements[1]) {
        return Err("Invalid segment: {segment}".into());
    }
    let segment = String::from(elements[1]);
    let i: u16 = elements[2].parse().unwrap();
    
    let pp = PushPop {
        segment,
        i
    };
    
    return match elements[0] {
        "pop" => {
            match pp.segment.as_str() {
                "constant" => Err("Invalid: pop constant".into()),
                _ => Ok(Command::Pop(pp)),
            }
        },
        "push" => Ok(Command::Push(pp)),
        _ => Err("Invalid command: {line}".into()),
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_arithmetic_command() {
        let input = "add";
        let want = Command::Arithmetic(Arithmetic {
            instruction: String::from("add"),
        });
        assert_eq!(want, parse_line(input).unwrap());

    }

    #[test]
    fn translates_pop_command() {
        let input = "pop this 6";
        let want = Command::Pop(PushPop {
            segment: String::from("this"),
            i: 6,
        });
        assert_eq!(want, parse_line(input).unwrap());
    }

    #[test]
    fn translates_push_command() {
        let input = "push this 6";
        let want = Command::Push(PushPop {
            segment: String::from("this"),
            i: 6,
        });
        assert_eq!(want, parse_line(input).unwrap());

    }

    #[test]
    fn errors_on_too_many_elements() {
        let input = "push push push this 6";
        assert!(parse_line(input).is_err());
    }

    #[test]
    fn errors_on_three_but_unknown() {
        let input = "pushi push push this 6";
        assert!(parse_line(input).is_err());
    }

    #[test]
    fn errors_on_pop_constant() {
        let input = "pop constant 6";
        assert!(handle_memory_command(input.split_whitespace().collect()).is_err());
    }

    #[test]
    fn errors_on_non_valid_push() {
        let input = "push pushback 6";
        assert!(handle_memory_command(input.split_whitespace().collect()).is_err());
    }

    #[test]
    fn errors_on_non_valid_arithmetic() {
        let input = "invalid";
        assert!(handle_arithmetic_command(input).is_err());
    }
}
