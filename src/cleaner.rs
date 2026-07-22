pub fn clean_program(lines: Vec<&str>) -> Vec<&str> {
    let mut instructions: Vec<&str> = vec![];
    let delimiter = "//";

    for l in lines {
        let code = match l.split_once(delimiter) {
            Some((before, _)) => before.trim(),
            None => l.trim(),
        };

        push_if_not_empty(&mut instructions, code);
    }

    instructions
}

fn push_if_not_empty<'a>(instructions: &mut Vec<&'a str>, line: &'a str) {
    if !line.is_empty() {
        instructions.push(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_full_comment_line() {
        let input = vec!["// Hi", "push constant 1"];
        assert_eq!(vec!["push constant 1"], clean_program(input));
    }

    #[test]
    fn strips_comment_after_line() {
        let input = vec!["push constant 1 // Hi"];
        assert_eq!(vec!["push constant 1"], clean_program(input));
    }

    #[test]
    fn strips_all_whitespace_around() {
        let input = vec!["      push constant 1     "];
        assert_eq!(vec!["push constant 1"], clean_program(input));
    }
}
