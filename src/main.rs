// Mul0 Hashing Algorithm
// =======================================
// https://github.com/mealet/mul0
// Project licensed under the MIT License.
// See more in LICENSE file.

mod algorithms;
mod cli;

use {algorithms::*, cli::Cli, std::io::Write, colored::Colorize};

fn main() {
    // Getting arguments
    let matches = Cli::command().get_matches();

    match matches.subcommand() {
        Some(("hash", sub_matches)) => {
            let input = sub_matches.get_one::<String>("STRING").expect("required");
            let output = hash!(&input.clone().bytes().collect::<Vec<u8>>());

            // Printing colored message to standard output, and clear output to stderr
            let _ = Cli::make_print(output);
        }
        Some(("dehash", sub_matches)) => {
            let input = sub_matches.get_one::<String>("STRING").expect("required");
            let output = dehash!(input.clone());

            // Printing colored message to standard output, and clear output to stderr
            let _ = Cli::make_print(String::from_utf8_lossy(&output).to_string());
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
                hash!(&input.bytes().collect::<Vec<u8>>())
            } else {
                String::from_utf8_lossy(&dehash!(input)).to_string()
            };

            let _ = Cli::make_print(output);
        }
        Some(("file", sub_matches)) => {
            let subcommand = sub_matches.subcommand().unwrap_or(("hash", sub_matches));

            match subcommand {
                ("hash", sub_matches) => {
                    let path_to_file = sub_matches.get_one::<String>("PATH").expect("required");
                    let file_bytes = Cli::get_file_bytes(path_to_file.clone());
                    let output = hash!(&file_bytes);

                    let mut new_file = std::fs::File::create(format!("{}.{}", path_to_file, cli::APP_NAME)).expect("Unable to use filesystem!");
                    let _ = new_file.write_all(&output.bytes().collect::<Vec<u8>>());

                    cli::successful!("File hashed!");
                },
                ("dehash", sub_matches) => {
                    let path_to_file = sub_matches.get_one::<String>("PATH").expect("required");
                    let file_hash = String::from_utf8_lossy(&Cli::get_file_bytes(path_to_file.clone())).to_string();
                    let mut output = dehash!(file_hash);

                    let path_fmt = format!("mul0-{}", path_to_file.replace(format!(".{}", cli::APP_NAME).as_str(), ""));
                    let mut output_file = std::fs::File::create(path_fmt).expect("Unable to use filysystem!");
                    let _ = output_file.write_all(&mut output);

                    cli::successful!("File dehashed!");
                },
                _ => unreachable!()
            }
        }
        _ => unreachable!(),
    }
}
