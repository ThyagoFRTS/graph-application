use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

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

pub fn create_file(content: String) {
    let mut data_file = File::create("graphviz.dot").expect("creation failed");
    // Write contents to the file
    data_file.write(content.as_str().as_bytes()).expect("write failed");
}

