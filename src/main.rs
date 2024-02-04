use clap::Command;

mod commands;

fn main() {
    let commands = commands::get_commands();
    let mut app = Command::new("Flare")
        .version("1.0")
        .author("Kellerman Rivero <krsloco@gmail.com>")
        .about("CLI utility for embedded devices");

    for command in commands.iter() {
        app = app.subcommand(command.get_definition());
    }

    let arg_matches = app.get_matches();
    commands::dispatch_command(arg_matches);
}
