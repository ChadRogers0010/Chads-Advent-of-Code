use std::{
    fs::{self, read_dir, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_file<P: AsRef<Path> + std::fmt::Debug + Clone + Copy>(
    path: P,
) -> Result<Vec<String>, std::io::Error> {
    // println!("Trying to find file");
    // let mut entries = fs::read_dir(path)?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;
    // println!("read dir: {:?}", entries);
    let file = File::open(path)?;
    println!("File Path: {:?}", file);
    let bufreader = BufReader::new(file).lines();
    let string_vec: Vec<String> = bufreader.map(|f| f.expect("Buffer failed")).collect();
    // println!("Input: {:?}", string_vec);
    Ok(string_vec)
}

// pub fn find_input<P: AsRef<Path>>() -> P {
//     let cur_loc = fs::ReadDir::from("./");
// }

#[derive(Debug)]
pub enum Part {
    One,
    Two,
    Both,
}

#[derive(Debug)]
pub enum SolutionResult<T> {
    Success(T),
    Failure(String),
    ReadWriteErr(Option<T>, std::io::Error),
}
