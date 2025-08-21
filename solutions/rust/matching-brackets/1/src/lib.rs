use std::collections::HashMap;


pub fn brackets_are_balanced(string: &str) -> bool {
    let bracket_map = HashMap::from([
        (')', '('), (']', '['), ('}', '{')
    ]);
    let mut stack = vec![];

    for c in string.chars() {
        if bracket_map.values().any(|&v| c == v) {
            stack.push(c);
        } else if bracket_map.contains_key(&c) {
            if stack.is_empty() { return false; }

            let last_bracket = stack.pop().unwrap();
            let matching_opened_bracket = bracket_map[&c];

            if last_bracket != matching_opened_bracket {
                return false;
            }
        }
    }
    stack.is_empty()
}
