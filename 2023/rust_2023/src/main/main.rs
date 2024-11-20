use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod cli;

fn main() {
    let _args = cli::build_cli();
    println!("Hello AoC 2023!");
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let bufreader = BufReader::new(file).lines();
    let string_vec: Vec<String> = bufreader.map(|f| f.unwrap()).collect();
    Ok(string_vec)
}
