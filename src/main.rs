use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Rows to select from input
    #[arg(short, long, allow_negative_numbers = true, required = false)]
    rows: String,

    /// Row delimiter
    #[arg(long, default_value = r"\n")]
    row_delimiter: String,

    /// Columns to select from input
    #[arg(short, long, allow_negative_numbers = true, required = false)]
    columns: String,

    /// Column delimiter
    #[arg(long, default_value = r"\s")]
    column_delimiter: String,

    /// Input text
    #[arg(value_delimiter = None, default_value = "", help="Text to parse")]
    input: String,
}

// Selector
#[derive(Debug)]
struct Selector {
    start_idx: usize,
    start_regex: regex::Regex,
    end_idx: usize,
    end_regex: regex::Regex,
    step: usize,
    stopped: bool,
}

impl Default for Selector {
    // Most of this is pretty self-exmplanatory
    // r".^"" is an impossible regex pattern that will always return false
    fn default() -> Selector {
        Selector {
            start_idx: 0,
            start_regex: Regex::new(r".^").unwrap(),
            end_idx: std::usize::MAX,
            end_regex: Regex::new(r".^").unwrap(),
            step: 1,
            stopped: false,
        }
    }
}

impl PartialEq for Selector {
    fn eq(&self, other: &Self) -> bool {
        self.start_idx == other.start_idx
            && self.start_regex.as_str() == other.start_regex.as_str()
            && self.end_idx == other.end_idx
            && self.end_regex.as_str() == other.end_regex.as_str()
            && self.step == other.step
            && self.stopped == other.stopped
    }
}

// Parse either column or row selectors
fn parse_selectors(selectors: &String) -> Vec<Selector> {
    let mut sequences: Vec<Selector> = Vec::new();
    for selector in selectors.split(",") {
        let mut sequence = Selector::default();
        for (idx, component) in selector.split(":").enumerate() {
            let parsed_component = component.parse::<usize>();
            match parsed_component {
                Ok(_ok) => {
                    let number = parsed_component.as_ref().unwrap() - 1;
                    match idx {
                        0 => {
                            sequence.start_idx = number;
                            if selector.matches(":").count() == 0 {
                                sequence.end_idx = number;
                            }
                        }
                        1 => sequence.end_idx = number,
                        2 => sequence.step = number,
                        _ => panic!("Cannot pass more than 3 components to a selector"),
                    }
                }
                Err(_e) => {
                    let case_insensitive_regex = format!(r"(?i).*{}.*", &component);
                    match idx {
                        0 => {
                            sequence.start_regex = Regex::new(&case_insensitive_regex).unwrap();
                            sequence.start_idx = usize::MAX;
                            if selector.matches(":").count() == 0 {
                                sequence.end_regex = Regex::new(&case_insensitive_regex).unwrap();
                            }
                        }
                        1 => sequence.end_regex = Regex::new(&case_insensitive_regex).unwrap(),
                        2 => panic!("Step must be a valid integer"),
                        _ => panic!("Cannot pass more than 3 components to a selector"),
                    }
                }
            }
        }
        sequences.push(sequence);
    }
    sequences
}

// Split text by a delimiter
fn split(text: &String, delimiter: &String) -> Vec<String> {
    if delimiter.is_empty() {
        text.lines()
            .filter(|&s| s.is_empty() == false)
            .map(String::from)
            .collect()
    } else {
        Regex::new(delimiter)
            .unwrap()
            .split(text)
            .filter(|&s| s.is_empty() == false)
            .map(String::from)
            .collect()
    }
}

// Get range of columns to use from header row
fn get_columns(
    index_row: &String,
    column_selectors: &mut Vec<Selector>,
    column_delimiter: &String,
) -> Vec<usize> {
    if column_selectors.len() == 0 {
        // Return blank vector if no column selectors present
        Vec::new()
    } else {
        // Return a vector of column indices to export
        let mut export_column_idxs: Vec<usize> = Vec::new();
        // Iterate through columns in first row
        for (col_idx, column) in split(index_row, column_delimiter).iter().enumerate() {
            // Iterate through selector in selectors
            for column_selector in &mut *column_selectors {
                if column_selector.stopped {
                    continue
                }
                let mut in_sequence: bool = false;
                if (col_idx == column_selector.start_idx
                    && (column_selector.start_regex.as_str() == ".^"))
                    || column_selector.start_regex.is_match(column)
                {
                    // Sequence start
                    export_column_idxs.push(col_idx);
                    column_selector.start_idx = col_idx;
                    in_sequence = true;
                    if (column_selector.end_regex.as_str() == column_selector.start_regex.as_str()
                        && (column_selector.start_regex.as_str() != ".^"))
                        || (column_selector.end_idx == column_selector.start_idx)
                    {
                        // Only one column selected
                        column_selector.stopped = true;
                    }
                } else if col_idx == column_selector.end_idx
                    || column_selector.end_regex.is_match(column)
                {
                    // Sequence end
                    export_column_idxs.push(col_idx);
                    column_selector.end_idx = col_idx;
                    in_sequence = true;
                } else if col_idx > column_selector.start_idx
                    && col_idx < column_selector.end_idx
                    && (col_idx - column_selector.start_idx) % column_selector.step == 0
                {
                    // Sequence middle
                    export_column_idxs.push(col_idx);
                    in_sequence = true;
                }
                // No need to check if row in other selectors once we add to export
                if in_sequence {
                    break;
                }
            }
        }
        export_column_idxs
    }
}

fn get_cells(row: &String, cols_to_select: &Vec<usize>, column_delimiter: &String) -> Vec<String> {
    if cols_to_select.len() == 0 {
        vec![(*row).clone()]
    } else {
        let mut output: Vec<String> = Vec::new();
        for (cell_idx, cell) in split(row, column_delimiter).iter().enumerate() {
            if cols_to_select.contains(&cell_idx) {
                output.push((*cell).clone());
            }
        }
        output
    }
}

fn main() {
    // Parse arguments
    let args = Args::parse();
    let input = if &args.input == "" {
        // If not input passed, read stdin (i.e. input from pipe)
        // Shoutout Frazer @ https://stackoverflow.com/a/73157621
        io::stdin()
            .lock()
            .lines()
            .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n")
            .to_string()
    } else if Path::new(&args.input).exists() {
        // If input string is an extant file, read its content as input
        fs::read_to_string(&args.input).expect("Input file could not be read.")
    } else {
        // If input string is present and not file, use it as input args.input
        args.input
    };
    let mut row_selectors = parse_selectors(&args.rows);
    let mut column_selectors = parse_selectors(&args.columns);

    // Parse input data according to arguments
    let mut export_cols: Vec<usize> = Vec::new();
    let mut output: Vec<Vec<String>> = Vec::new();
    let split_rows = split(&input, &args.row_delimiter);
    for (row_idx, row) in split_rows.iter().enumerate() {
        if row_idx == 0 {
            export_cols = get_columns(row, &mut column_selectors, &args.column_delimiter);
        }
        for row_selector in &mut row_selectors {
            if row_selector.stopped {
                continue
            }
            let mut in_sequence: bool = false;
            if (row_idx == row_selector.start_idx && (row_selector.start_regex.as_str() == ".^"))
                || row_selector.start_regex.is_match(row)
            {
                // Sequence start
                output.push(get_cells(&row, &export_cols, &args.column_delimiter));
                row_selector.start_idx = row_idx;
                in_sequence = true;
                if (row_selector.end_regex.as_str() == row_selector.start_regex.as_str()
                    && (row_selector.start_regex.as_str() != ".^"))
                    || (row_selector.end_idx == row_selector.start_idx)
                {
                    // Only one column selected
                    row_selector.stopped = true;
                }
            } else if row_idx == row_selector.end_idx || row_selector.end_regex.is_match(row) {
                // Sequence end
                output.push(get_cells(&row, &export_cols, &args.column_delimiter));
                row_selector.end_idx = row_idx;
                in_sequence = true;
            } else if row_idx > row_selector.start_idx
                && row_idx < row_selector.end_idx
                && (row_idx - row_selector.start_idx) % row_selector.step == 0
            {
                // Sequence middle
                output.push(get_cells(&row, &export_cols, &args.column_delimiter));
                in_sequence = true;
            }
            // No need to check if row in other selectors once we add to export
            if in_sequence {
                break;
            }
        }
    }
    for line in output {
        println!("{}", line.join("\t"));
    }
}
