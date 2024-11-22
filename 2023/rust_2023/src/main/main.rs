use cli::const_file::*;
use lib::utils::Part;
mod cli;

fn main() {
    let mut cli = cli::build_cli();
    let arg_matches = cli.clone().get_matches();
    // let part: Part = match &arg_matches {
    //     flag if flag.get_flag("o") => Part::One,
    //     flag if flag.get_flag("t") => Part::Two,
    //     flag if flag.get_flag("b") => Part::Both,
    //     _ => panic!("no flags"),
    // };

    if !arg_matches.args_present() {
        println!("No args present");
        let _ = cli.print_long_help();
    }

    let part = Part::One;
    // let part = match &arg_matches {
    //     part if part
    //         // .subcommand_matches(PROGRAM_NAME)
    //         // .unwrap()
    //         .try_get_one::<&String>(PART_ONE)
    //         .is_ok() =>
    //     {
    //         Part::One
    //     }
    //
    //     part if part
    //         // .subcommand_matches(PROGRAM_NAME)
    //         // .unwrap()
    //         .try_get_one::<&String>(PART_TWO)
    //         .is_ok() =>
    //     {
    //         Part::Two
    //     }
    //
    //     part if part
    //         // .subcommand_matches(PROGRAM_NAME)
    //         // .unwrap()
    //         .contains_id()
    //         .try_get_one::<&String>(PART_BOTH)
    //         .is_ok() =>
    //     {
    //         Part::Both
    //     }
    //     _ => todo!(),
    // };
    println!("part: {:?}", part);

    match &arg_matches {
        day_command if day_command.subcommand_matches(DAY_01).is_some() => {
            println!("{} subcommand passed!", DAY_01);
            lib::solutions::day_one::day_one_solver(part);
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
    };
}
