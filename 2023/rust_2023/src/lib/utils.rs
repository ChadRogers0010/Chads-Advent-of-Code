use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let bufreader = BufReader::new(file).lines();
    let string_vec: Vec<String> = bufreader.map(|f| f.expect("Buffer failed")).collect();
    // println!("Input: {:?}", string_vec);
    Ok(string_vec)
}

// pub fn find_input<P: AsRef<Path>>() -> P {
//     let cur_loc = fs::ReadDir::from("./");
// }

pub enum Part {
    One,
    Two,
    Both,
}
