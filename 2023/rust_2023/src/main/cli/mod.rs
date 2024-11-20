use clap::Command;
use clap::*;

use const_file::*;
pub mod const_file;
pub fn build_cli() -> clap::Command {
    Command::new(PROGRAM_NAME)
        .args([arg!(-o - -one), arg!(-t - -two), arg!(-b - -both)])
        .command_01()
        .command_02()
        .command_03()
        .command_04()
        .command_05()
        .command_06()
        .command_07()
        .command_08()
        .command_09()
        .command_10()
        .command_11()
        .command_12()
        .command_13()
        .command_14()
        .command_15()
        .command_16()
        .command_17()
        .command_18()
        .command_19()
        .command_20()
        .command_21()
        .command_22()
        .command_23()
        .command_24()
        .command_25()
}

trait ExtendCommand {
    fn command_01(self) -> Command;
    fn command_02(self) -> Command;
    fn command_03(self) -> Command;
    fn command_04(self) -> Command;
    fn command_05(self) -> Command;
    fn command_06(self) -> Command;
    fn command_07(self) -> Command;
    fn command_08(self) -> Command;
    fn command_09(self) -> Command;
    fn command_10(self) -> Command;
    fn command_11(self) -> Command;
    fn command_12(self) -> Command;
    fn command_13(self) -> Command;
    fn command_14(self) -> Command;
    fn command_15(self) -> Command;
    fn command_16(self) -> Command;
    fn command_17(self) -> Command;
    fn command_18(self) -> Command;
    fn command_19(self) -> Command;
    fn command_20(self) -> Command;
    fn command_21(self) -> Command;
    fn command_22(self) -> Command;
    fn command_23(self) -> Command;
    fn command_24(self) -> Command;
    fn command_25(self) -> Command;
}

fn init_command(name: &'static str) -> Command {
    Command::new(name)
}

impl ExtendCommand for Command {
    fn command_01(self) -> Command {
        self.subcommand(init_command(DAY_01))
    }
    fn command_02(self) -> Command {
        self.subcommand(init_command(DAY_02))
    }
    fn command_03(self) -> Command {
        self.subcommand(init_command(DAY_03))
    }
    fn command_04(self) -> Command {
        self.subcommand(init_command(DAY_04))
    }
    fn command_05(self) -> Command {
        self.subcommand(init_command(DAY_05))
    }
    fn command_06(self) -> Command {
        self.subcommand(init_command(DAY_06))
    }
    fn command_07(self) -> Command {
        self.subcommand(init_command(DAY_07))
    }
    fn command_08(self) -> Command {
        self.subcommand(init_command(DAY_08))
    }
    fn command_09(self) -> Command {
        self.subcommand(init_command(DAY_09))
    }
    fn command_10(self) -> Command {
        self.subcommand(init_command(DAY_10))
    }
    fn command_11(self) -> Command {
        self.subcommand(init_command(DAY_11))
    }
    fn command_12(self) -> Command {
        self.subcommand(init_command(DAY_12))
    }
    fn command_13(self) -> Command {
        self.subcommand(init_command(DAY_13))
    }
    fn command_14(self) -> Command {
        self.subcommand(init_command(DAY_14))
    }
    fn command_15(self) -> Command {
        self.subcommand(init_command(DAY_15))
    }
    fn command_16(self) -> Command {
        self.subcommand(init_command(DAY_16))
    }
    fn command_17(self) -> Command {
        self.subcommand(init_command(DAY_17))
    }
    fn command_18(self) -> Command {
        self.subcommand(init_command(DAY_18))
    }
    fn command_19(self) -> Command {
        self.subcommand(init_command(DAY_19))
    }
    fn command_20(self) -> Command {
        self.subcommand(init_command(DAY_20))
    }
    fn command_21(self) -> Command {
        self.subcommand(init_command(DAY_21))
    }
    fn command_22(self) -> Command {
        self.subcommand(init_command(DAY_22))
    }
    fn command_23(self) -> Command {
        self.subcommand(init_command(DAY_23))
    }
    fn command_24(self) -> Command {
        self.subcommand(init_command(DAY_24))
    }
    fn command_25(self) -> Command {
        self.subcommand(init_command(DAY_25))
    }
}
