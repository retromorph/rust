#![forbid(unsafe_code)]

pub fn is_correct_bracket_sequence(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        if ch == '(' || ch == '[' || ch == '{' {
            stack.push(ch);
            continue;
        }

        match stack.pop() {
            None => return false,
            Some('(') if ch != ')' => return false,
            Some('[') if ch != ']' => return false,
            Some('{') if ch != '}' => return false,
            _ => {}
        }
    }

    stack.is_empty()
}
