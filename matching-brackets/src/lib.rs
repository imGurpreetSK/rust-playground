pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let brackets = ('[',']');
    let parentheses = ('(',')');
    let braces = ('{','}');

    for c in string.chars() {
        if c == braces.0 || c == brackets.0 || c == parentheses.0 {
            stack.push(c);
        } else {
            if c == braces.1 && stack.last() == Some(&braces.0) { stack.pop().unwrap(); }
            else if c == brackets.1 && stack.last() == Some(&brackets.0) { stack.pop().unwrap(); }
            else if c == parentheses.1 && stack.last() == Some(&parentheses.0) { stack.pop().unwrap(); }
            else if c != braces.1 && c != brackets.1 && c != parentheses.1 { continue; }
            else { return false; }
        }
    };

    stack.is_empty()
}
