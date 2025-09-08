use clap::Parser;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// CLI arguments parsed here
/// All parsing handled by the `clap` crate
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Rows to select from input
    #[arg(short, long, allow_negative_numbers = true, default_value = "")]
    pub rows: String,

    /// Row delimiter
    #[arg(long, default_value = r"\n")]
    pub row_delimiter: String,

    /// Columns to select from input
    #[arg(short, long, allow_negative_numbers = true, default_value = "")]
    pub columns: String,

    /// Column delimiter
    #[arg(long, default_value = r"\s")]
    pub column_delimiter: String,

    /// Text to parse
    #[arg(value_delimiter = None, default_value = "", help = "Text to parse")]
    pub input: String,
}

/// Read String from stdin (allow piped input)
/// Shoutout to Frazer's Stack Overflow answer (https://stackoverflow.com/a/73157621)
fn read_stdin() -> String {
    io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n")
        .to_string()
}

/// Parse input, allowing file, piped text, or text as an argument
pub fn parse_input(input_text: &str) -> String {
    if input_text.is_empty() {
        // If not input passed, read stdin (i.e. input from pipe)
        read_stdin()
    } else if Path::new(input_text).exists() {
        // If input string is an extant file, read its content as input
        fs::read_to_string(input_text).expect("Input file could not be read.")
    } else {
        // If input string is present and not file, use it as input args.input
        input_text.to_string()
    }
}

#[cfg(test)]
#[path = "cli_tests.rs"]
mod cli_tests;
