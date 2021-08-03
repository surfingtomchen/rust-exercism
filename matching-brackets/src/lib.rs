pub fn brackets_are_balanced(string: &str) -> bool {
    let pair_char = |c| match c {
        '}' => '{',
        ')' => '(',
        ']' => '[',
        _ => c
    };

    let mut stack: Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            '}' | ')' | ']' => {
                if let Some(last) = stack.pop() {
                    if last != pair_char(c) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}
