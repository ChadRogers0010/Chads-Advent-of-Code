use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    println!("Hello, world!");
}

pub fn part_one_solver(strings: &[String]) -> i32 {
    strings
        .iter()
        .map(|f| -> (char, char) {
            let mut a: Option<char> = None;
            let mut b: Option<char> = None;
            for char in f.chars() {
                if match_char_number(char) {
                    a = Some(char);
                    break;
                }
            }
            for char in f.chars().rev() {
                if match_char_number(char) {
                    b = Some(char);
                    break;
                }
            }
            (a.unwrap(), b.unwrap())
        })
        .map(|f| -> i32 { format!("{}{}", f.0, f.1).parse().unwrap() })
        .sum()
}

pub fn part_two_solver(_input: &[String]) -> i32 {
    todo!()
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let bufreader = BufReader::new(file).lines();
    let string_vec: Vec<String> = bufreader.map(|f| f.unwrap()).collect();
    Ok(string_vec)
}

pub fn match_char_number(char: char) -> bool {
    match char {
        char if char == '0' => true,
        char if char == '1' => true,
        char if char == '2' => true,
        char if char == '3' => true,
        char if char == '4' => true,
        char if char == '5' => true,
        char if char == '6' => true,
        char if char == '7' => true,
        char if char == '8' => true,
        char if char == '9' => true,
        _ => false,
    }
}
