mod algorithms;
mod cli;

use {algorithms::Mul0, cli::Cli, colored::Colorize, std::io::Write};

fn main() {
    // getting arguments
    let matches = Cli::command().get_matches();

    match matches.subcommand() {
        Some(("hash", sub_matches)) => {
            let input = sub_matches.get_one::<String>("STRING").expect("required");
            let output = Mul0::hash(input.clone());

            // Printing colored message to stdout, and clear output to stderr
            let _ = Cli::make_print(input.clone(), output);
        }
        Some(("dehash", sub_matches)) => {
            let input = sub_matches.get_one::<String>("STRING").expect("required");
            let output = Mul0::dehash(input.clone());

            // Printing colored message to stdout, and clear output to stderr
            let _ = Cli::make_print(input.clone(), output);
        }
        Some(("manual", _)) => {
            // Interactive mode
            // Getting algorithm mode (0 - hash, 1 - dehash)
            print!(
                "{}",
                "Choose algorithm mode.\n0 - hash (default), 1 - dehash: ".truecolor(252, 73, 3)
            );
            let _ = std::io::stdout().flush();

            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);

            let mode = buffer
                .trim()
                .parse::<u8>()
                .expect("Unable to read chosen mode");

            // Reading input
            print!("{}", "Type you'r input: ".truecolor(252, 73, 3));

            let mut buffer = String::new();
            let _ = std::io::stdout().flush();
            let _ = std::io::stdin().read_line(&mut buffer);

            let input = buffer.trim();

            // Hashing and doing output
            let output = if mode == 0 {
                Mul0::hash(input.to_string())
            } else {
                Mul0::dehash(input.to_string())
            };

            let _ = Cli::make_print(input.to_string(), output);
        }
        _ => unreachable!(),
    }
}

// FIXME: Optimize and make code some cleaner
