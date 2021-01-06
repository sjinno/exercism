pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_stack = vec![];
    for c in string.chars().into_iter() {
        match c {
            c if c == '(' || c == '[' || c == '{' => {
                bracket_stack.push(c);
            }
            c if c == ')' || c == ']' || c == '}' => {
                if bracket_stack.is_empty() {
                    return false;
                }
                let bracket = bracket_stack.pop().unwrap();
                if c == ')' && bracket != '(' {
                    return false;
                } else if c == ']' && bracket != '[' {
                    return false;
                } else if c == '}' && bracket != '{' {
                    return false;
                }
            }
            _ => continue,
        }
    }
    bracket_stack.is_empty()
}
