// Mul0 Hashing Algorithm
// =======================================
// https://github.com/mealet/mul0
// Project licensed under the MIT License.
// See more in LICENSE file.

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const ACCIENT_COLOR: (u8, u8, u8) = (28, 153, 255);

use clap::{arg, Command};
use colored::Colorize;
use std::io::{stdin, stdout, Write, Read};

macro_rules! error {
    ($input:expr) => {
        println!("{} {}", "error:".red(), $input)
    };
}

macro_rules! successful {
    ($msg:expr) => {
        println!("{} {}", "successful:".green(), $msg)
    };
}

pub(crate) use {error, successful};

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
            .subcommand(
                Command::new("file")
                    .about("Provides user to hash and dehash file")
                    .args_conflicts_with_subcommands(true)
                    .flatten_help(true)
                    .subcommand(Command::new("hash").arg(arg!(<PATH>)))
                    .subcommand(Command::new("dehash").arg(arg!(<PATH>)))
            )
            .subcommand(Command::new("manual").about("Manual interactive mode"))
    }

    pub fn get_file_bytes(path: String) -> Vec<u8> {
        let mut file = std::fs::File::open(path.clone()).unwrap_or_else(|_| {
            error!(format!("File `{}` not found!", path));
            std::process::exit(1);
        });

        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer).unwrap_or_else(|_| {
            error!(format!("Cannot read file `{}`", path));
            std::process::exit(1);
        });

        return buffer;
    }

    pub fn make_print(output: String) {
        // Printing colored message in standard output, and returning clear result into stderr
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
