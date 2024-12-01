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
        let mut cli = self;
        for year in YEAR_ARRAY {
            let subcom = clap::Command::new(year).init_days();
            cli = cli.subcommand(subcom);
        }
        cli
    }

    fn init_days(self) -> Self {
        let mut cli = self;
        for day in DAY_ARRAY {
            let arg = Arg::new(day); // .action(clap::ArgAction::SetTrue);
            cli = cli.arg(arg);
        }
        cli
    }
}
