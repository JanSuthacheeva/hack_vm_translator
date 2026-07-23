pub fn clean_program(program: &str) -> Vec<&str> {
    let mut instructions: Vec<&str> = vec![];
    let delimiter = "//";

    for line in program.lines() {
        let code = match line.split_once(delimiter) {
            Some((before, _)) => before.trim(),
            None => line.trim(),
        };

        if !code.is_empty() {
            instructions.push(code);
        }
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_full_comment_line() {
        let input = "// Hi\n push constant 1";
        assert_eq!(vec!["push constant 1"], clean_program(input));
    }

    #[test]
    fn strips_comment_after_line() {
        let input = "push constant 1 // Hi";
        assert_eq!(vec!["push constant 1"], clean_program(input));
    }

    #[test]
    fn strips_all_whitespace_around() {
        let input = "      push constant 1     ";
        assert_eq!(vec!["push constant 1"], clean_program(input));
    }
}
