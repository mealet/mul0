// Mul0 Hashing Algorithm
// =======================================
// https://github.com/mealet/mul0
// Project licensed under the MIT License.
// See more in LICENSE file.

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const ACCIENT_COLOR: (u8, u8, u8) = (28, 153, 255);

use clap::{arg, Command};
use colored::Colorize;
use std::io::{stdin, stdout, Write};

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

    pub fn make_print(output: String) {
        // Printing colored message in stdout, and returning clear result into stderr
        println!(
            "{}",
            "Mul0 output (printed to stderr):".truecolor(
                ACCIENT_COLOR.0,
                ACCIENT_COLOR.1,
                ACCIENT_COLOR.2
            )
        );
        eprint!("{}", output);
        println!("");
    }

    pub fn get_input(display: String) -> String {
        let mut buffer = String::new();

        print!(
            "{}",
            display.truecolor(ACCIENT_COLOR.0, ACCIENT_COLOR.1, ACCIENT_COLOR.2)
        );
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut buffer);

        return buffer.trim().to_string();
    }
}
