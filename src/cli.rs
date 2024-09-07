const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

use clap::{arg, Command};
use colored::Colorize;

pub struct Cli;

impl Cli {
    pub fn command() -> Command {
        Command::new(APP_NAME)
            .about("A simple hashing algorithm based on multiplying bytes")
            .version(APP_VERSION)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .allow_external_subcommands(true)
            .subcommand(
                Command::new("hash")
                    .about("Hashes you'r input")
                    .arg(arg!(<STRING> "String input from user"))
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("dehash")
                    .about("Returns hashed string's source")
                    .arg(arg!(<STRING> "String input from user")),
            )
            .subcommand(Command::new("manual").about("Manual interactive mode"))
    }

    pub fn make_print(input: String, output: String) {
        // Printing colored message in stdout, and returning clear result into stderr
        println!(
            "{}\n{}",
            "Provided user input:".truecolor(252, 73, 3),
            input
        );
        println!(
            "{}",
            "Mul0 hashed output (printed to stderr):".truecolor(252, 73, 3)
        );
        eprint!("{}", output);
        println!("");
    }
}
