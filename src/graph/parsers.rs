use std::io::{self, Write};

pub fn parse_input_vec(buf : &mut String) -> Vec<u32> {
    let ret = buf
        .split_whitespace()
        .map(|token| token.parse().expect("Failed to parse."))
        .collect();
    buf.clear();
    ret
}

pub fn parse_input_usz(buf : &mut String) -> usize {
    let ret = buf.trim().parse().expect("Invalid input.");
    buf.clear();
    ret
}

pub fn input(prompt: &str, buf : &mut String) {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(buf)
        .expect("Failed to input.");
}