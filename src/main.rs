// Mul0 Hashing Algorithm
// =======================================
// https://github.com/mealet/mul0
// Project licensed under the MIT License.
// See more in LICENSE file.

mod algorithms;
mod cli;

use {algorithms::*, cli::Cli};

fn main() {
    // getting arguments
    let matches = Cli::command().get_matches();

    match matches.subcommand() {
        Some(("hash", sub_matches)) => {
            let input = sub_matches.get_one::<String>("STRING").expect("required");
            let output = hash!(input.clone());

            // Printing colored message to stdout, and clear output to stderr
            let _ = Cli::make_print(output);
        }
        Some(("dehash", sub_matches)) => {
            let input = sub_matches.get_one::<String>("STRING").expect("required");
            let output = dehash!(input.clone());

            // Printing colored message to stdout, and clear output to stderr
            let _ = Cli::make_print(output);
        }
        Some(("manual", _)) => {
            // Interactive mode
            // Getting algorithm mode (0 - hash, 1 - dehash)
            let mode = Cli::get_input(
                "Choose algorithm mode.\n0 - hash (default), 1 - dehash: ".to_string(),
            );

            let mode = mode
                .trim()
                .parse::<u8>()
                .expect("Unable to read chosen mode");

            // Reading input
            let input = Cli::get_input("Type you'r input: ".to_string());

            // Hashing and doing output
            let output = if mode == 0 {
                hash!(input.to_string())
            } else {
                dehash!(input.to_string())
            };

            let _ = Cli::make_print(output);
        }
        _ => unreachable!(),
    }
}
