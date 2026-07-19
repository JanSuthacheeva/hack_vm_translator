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
    segment: String,
    i: u16,
}

#[derive(PartialEq, Debug)]
pub struct Arithmetic {
    instruction: String,
}

fn parse_line(line: &str) -> Result<Command, Box<dyn Error>> {

    let elements = line.split_whitespace().count();

    match elements {
        1 => Ok(Command::Arithmetic(
            Arithmetic {
                instruction: String::from(line)
            }
            )),
        3 => {
            let elements: Vec<&str> = line.split_whitespace().collect();
            let segment = String::from(elements[1]);
            let i: u16 = elements[2].parse().unwrap();

            let pp = PushPop {
                segment,
                i
            };

            return match elements[0] {
                "pop" => Ok(Command::Pop(pp)),
                "push" => Ok(Command::Push(pp)),
                _ => Err("Invalid command: {line}".into()),
            };
        }
        _ => Err("Invalid command: {line}".into()),
    }


}

pub fn parse(program: Vec<&str>) -> Result<Vec<Command>, Box<dyn Error>> {

    let mut result: Vec<Command> = vec![];
    for line in program {
        let command = parse_line(line)?;
        result.push(command);
    }

    Ok(result)
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
}
