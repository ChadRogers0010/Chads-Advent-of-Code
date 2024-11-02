use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[cfg(test)]
mod tests {}

pub fn solver(strings: Vec<String>) -> i32 {
    let mut results_vec = Vec::with_capacity(strings.len());
    for each in strings {}
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let bufreader = BufReader::new(file).lines();
    let string_vec: Vec<String> = bufreader.map(|f| f.unwrap()).collect();
    Ok(string_vec)
}

pub fn match_char_number(char: char) -> Option<i32> {
    match char {
        char if char == '0' => Some(0),
        char if char == '1' => Some(1),
        char if char == '2' => Some(2),
        char if char == '3' => Some(3),
        char if char == '4' => Some(4),
        char if char == '5' => Some(5),
        char if char == '6' => Some(6),
        char if char == '7' => Some(7),
        char if char == '8' => Some(8),
        char if char == '9' => Some(9),
        _ => None,
    }
}
