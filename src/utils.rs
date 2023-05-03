use std::collections::HashMap;

pub fn remove_letters(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_numeric() || c == ',' {
            result.push(c);
        }
    }
    result
}

pub fn create_mapper(v: Vec<usize>) -> impl Fn(usize) -> Option<usize> {
    let mut mapper = HashMap::new();
    for (i, &x) in v.iter().enumerate() {
        mapper.insert(x, i);
    }
    move |value| mapper.get(&value).cloned()
}

