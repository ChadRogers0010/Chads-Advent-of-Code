use clap::Command;
use clap::*;
use const_file::*;
pub mod const_file;
pub fn build_cli() -> Command {
    Command::new(PROGRAM_NAME).init_years()
}

trait ExtendCommand {
    fn init_years(self) -> Self;
    fn init_days(self) -> Self;
}

impl ExtendCommand for clap::Command {
    fn init_years(self) -> Self {
        let mut scratch = self;
        for year in YEAR_ARRAY {
            let subcom = clap::Command::new(year).init_days().about(year);
            scratch = scratch.subcommand(subcom);
        }
        scratch
    }

    fn init_days(self) -> Self {
        let mut scratch = self;
        for day in DAY_ARRAY {
            scratch = scratch.arg(Arg::new(day));
        }
        scratch
    }
}
