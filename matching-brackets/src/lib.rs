use std::collections::{HashMap, HashSet};

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let closing = HashMap::from([('}', '{'), (')', '('), (']', '[')]);
    let valid_chars: HashSet<char> = HashSet::from(['(', ')', '{', '}', '[', ']']);

    for c in string.chars().filter(|c| valid_chars.contains(&c)) {
        match c {
            '(' | '{' | '[' => stack.push(c),
            _ => {
                if stack.iter().last() == closing.get(&c) {
                    stack.pop();
                } else {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}
