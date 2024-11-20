use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use cli::const_file::*;

mod cli;

fn main() {
    let arg_matches = cli::build_cli().get_matches();

    match &arg_matches {
        day_command if day_command.subcommand_matches(DAY_01).is_some() => {
            println!("{} subcommand passed!", DAY_01);
        }
        day_command if day_command.subcommand_matches(DAY_02).is_some() => {
            println!("{} subcommand passed!", DAY_02);
        }
        day_command if day_command.subcommand_matches(DAY_03).is_some() => {
            println!("{} subcommand passed!", DAY_03);
        }
        day_command if day_command.subcommand_matches(DAY_04).is_some() => {
            println!("{} subcommand passed!", DAY_04);
        }
        day_command if day_command.subcommand_matches(DAY_05).is_some() => {
            println!("{} subcommand passed!", DAY_05);
        }
        day_command if day_command.subcommand_matches(DAY_06).is_some() => {
            println!("{} subcommand passed!", DAY_06);
        }
        day_command if day_command.subcommand_matches(DAY_07).is_some() => {
            println!("{} subcommand passed!", DAY_07);
        }
        day_command if day_command.subcommand_matches(DAY_08).is_some() => {
            println!("{} subcommand passed!", DAY_08);
        }
        day_command if day_command.subcommand_matches(DAY_09).is_some() => {
            println!("{} subcommand passed!", DAY_09);
        }
        day_command if day_command.subcommand_matches(DAY_10).is_some() => {
            println!("{} subcommand passed!", DAY_10);
        }
        day_command if day_command.subcommand_matches(DAY_11).is_some() => {
            println!("{} subcommand passed!", DAY_11);
        }
        day_command if day_command.subcommand_matches(DAY_12).is_some() => {
            println!("{} subcommand passed!", DAY_12);
        }
        day_command if day_command.subcommand_matches(DAY_13).is_some() => {
            println!("{} subcommand passed!", DAY_13);
        }
        day_command if day_command.subcommand_matches(DAY_14).is_some() => {
            println!("{} subcommand passed!", DAY_14);
        }
        day_command if day_command.subcommand_matches(DAY_15).is_some() => {
            println!("{} subcommand passed!", DAY_15);
        }
        day_command if day_command.subcommand_matches(DAY_16).is_some() => {
            println!("{} subcommand passed!", DAY_16);
        }
        day_command if day_command.subcommand_matches(DAY_17).is_some() => {
            println!("{} subcommand passed!", DAY_17);
        }
        day_command if day_command.subcommand_matches(DAY_18).is_some() => {
            println!("{} subcommand passed!", DAY_18);
        }
        day_command if day_command.subcommand_matches(DAY_19).is_some() => {
            println!("{} subcommand passed!", DAY_19);
        }
        day_command if day_command.subcommand_matches(DAY_20).is_some() => {
            println!("{} subcommand passed!", DAY_20);
        }
        day_command if day_command.subcommand_matches(DAY_21).is_some() => {
            println!("{} subcommand passed!", DAY_21);
        }
        day_command if day_command.subcommand_matches(DAY_22).is_some() => {
            println!("{} subcommand passed!", DAY_22);
        }
        day_command if day_command.subcommand_matches(DAY_23).is_some() => {
            println!("{} subcommand passed!", DAY_23);
        }
        day_command if day_command.subcommand_matches(DAY_24).is_some() => {
            println!("{} subcommand passed!", DAY_24);
        }
        day_command if day_command.subcommand_matches(DAY_25).is_some() => {
            println!("{} subcommand passed!", DAY_25);
        }
        _ => println!("No commands given"),
    };
    println!("Hello AoC 2023!");
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let bufreader = BufReader::new(file).lines();
    let string_vec: Vec<String> = bufreader.map(|f| f.unwrap()).collect();
    Ok(string_vec)
}
