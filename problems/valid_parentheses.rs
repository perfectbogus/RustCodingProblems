
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => { stack.push(c) },
            ')' => if stack.pop() != Some('(') { return false },
            '}' => if stack.pop() != Some('{') { return false },
            ']' => if stack.pop() != Some('[') { return false },
            _ => return false
        }
    }

    stack.is_empty()
}
fn main() {
    let test_cases = vec![
        "()".to_string(),
        "()[]{}".to_string(),
        "(]".to_string(),
        "([)]".to_string(),
        "{[]}".to_string(),
    ];

    for case in test_cases {
        println!("Input: {}", case);
        println!("Output: {}", is_valid(case));
        println!();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_simple_parentheses() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn test_valid_mixed_brackets() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_invalid_mismatched_brackets() {
        assert!(!is_valid("(]".to_string()));
    }

    #[test]
    fn test_invalid_wrongly_ordered_brackets() {
        assert!(!is_valid("([)]".to_string()));
    }

    #[test]
    fn test_valid_nested_brackets() {
        assert!(is_valid("{[]}".to_string()));
    }

    #[test]
    fn test_invalid_unclosed_bracket() {
        assert!(!is_valid("(".to_string()));
    }

    #[test]
    fn test_invalid_only_closing_bracket() {
        assert!(!is_valid(")".to_string()));
    }

    #[test]
    fn test_empty_string() {
        assert!(is_valid("".to_string()));
    }
}