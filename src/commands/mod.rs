use clap::{ArgMatches, Command};

mod screencontrol;

pub trait FlareCommand {
    fn get_definition(&self) -> Command;
    fn execute(&self, arg_matches: &ArgMatches);
}

pub fn get_commands() -> Vec<Box<dyn FlareCommand>> {
    let mut commands: Vec<Box<dyn FlareCommand>> = Vec::new();
    commands.push(Box::new(screencontrol::ScreenControlCommand()));
    commands
}

pub fn dispatch_command(arg_matches: ArgMatches) -> bool {
    match arg_matches.subcommand() {
        Some((screencontrol::COMMAND_NAME, args)) => {
            let cmd = screencontrol::ScreenControlCommand();
            cmd.execute(args);
            true
        }
        _ => {
            println!("Please use flare --help to see commands available");
            false
        }
    }
}