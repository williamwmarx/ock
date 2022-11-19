use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// CLI arguments parsed here
/// All parsing handled by the `clap` crate
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

    /// Text to parse
    #[arg(value_delimiter = None, default_value = "", help="Text to parse")]
    input: String,
}

/// Keep track of user column and row selections
#[derive(Debug)]
struct Selector {
    /// Index of first row to grab (start of range)
    start_idx: usize,

    /// Regex of first to grab (start of range)
    start_regex: regex::Regex,

    /// Index of last row to grab (end of range)
    end_idx: usize,

    /// Regex of last row to grab (end of range)
    end_regex: regex::Regex,

    /// Step size between start and end of range
    step: usize,

    /// Keep track of when to stop adding rows from range to output
    stopped: bool,
}

impl Default for Selector {
    /// Defaults to implement a new selector without defining each field individually
    fn default() -> Selector {
        Selector {
            /// Default start to 0, the first row/column
            start_idx: 0,

            /// Default start to ".^", an impossible Regex that nothing will match
            start_regex: Regex::new(r".^").unwrap(),

            /// Default end to the max usize value (i.e. 2^64 - 1 on an amd64 machine)
            end_idx: std::usize::MAX,

            /// Default end to ".^", an impossible Regex that nothing will match
            end_regex: Regex::new(r".^").unwrap(),

            /// Default step to 1 to get each row
            step: 1,

            /// Default stopped to false so we output rows unless otherwise specified
            stopped: false,
        }
    }
}

impl PartialEq for Selector {
    /// Enable checking the equality of two Selector structs
    /// We do this by simply ensuring each field in the structs are equal
    /// While this seems straight forward, it's necessary as `regex::Regex` does not have a
    /// PartialEq implemented by default.
    fn eq(&self, other: &Self) -> bool {
        self.start_idx == other.start_idx
            && self.start_regex.as_str() == other.start_regex.as_str()
            && self.end_idx == other.end_idx
            && self.end_regex.as_str() == other.end_regex.as_str()
            && self.step == other.step
            && self.stopped == other.stopped
    }
}

/// Parse either row or column selectors, turning Python-like list slicing syntax into vector of
/// Selector structs
fn parse_selectors(selectors: &String) -> Vec<Selector> {
    let mut sequences: Vec<Selector> = Vec::new();
    // Iterate through selectors, which are separated by commas
    for selector in selectors.split(",") {
        let mut sequence = Selector::default();
        // Iterate through components in an individual selector, which are separated by colons
        for (idx, component) in selector.split(":").enumerate() {
            // Try to parse int from component. If we're successful, use that int as a start index,
            // end index, or step. If parse() returns an error, use that component as a regex
            // pattern to match to
            let parsed_component = component.parse::<usize>();
            match parsed_component {
                Ok(_ok) => {
                    // Subtract 1 from row, so 1:10 selects rows 1 to 10, not 2 to 11
                    let number = parsed_component.as_ref().unwrap() - 1;
                    match idx {
                        0 => {
                            sequence.start_idx = number;
                            // If this is the full selection, set this to the end index as well
                            if selector.matches(":").count() == 0 {
                                sequence.end_idx = number;
                            }
                        }
                        1 => sequence.end_idx = number,
                        2 => sequence.step = number,
                        _ => panic!("A selector cannot be more than three components long"),
                    }
                }
                Err(_e) => {
                    let case_insensitive_regex = format!(r"(?i).*{}.*", &component);
                    match idx {
                        0 => {
                            sequence.start_regex = Regex::new(&case_insensitive_regex).unwrap();
                            // Set the start index to the usize max to ensure it doesn't interfere
                            sequence.start_idx = usize::MAX;
                            // If this is the full selection, set this to the end regex as well
                            if selector.matches(":").count() == 0 {
                                sequence.end_regex = Regex::new(&case_insensitive_regex).unwrap();
                            }
                        }
                        1 => sequence.end_regex = Regex::new(&case_insensitive_regex).unwrap(),
                        2 => panic!("Step size must be an integer"),
                        _ => panic!("A selector cannot be more than three components long"),
                    }
                }
            }
        }
        // Add parsed selector to vector
        sequences.push(sequence);
    }
    // Return all selectors
    sequences
}

/// Split given text by a delimiter, returning a vector of Strings
fn split(text: &String, delimiter: &String) -> Vec<String> {
    if delimiter.is_empty() {
        // Split by lines if empty delmiter passed. This should be faster than regex split
        text.lines()
            .filter(|&s| s.is_empty() == false)
            .map(String::from)
            .collect()
    } else {
        // Split by regex
        Regex::new(delimiter)
            .unwrap()
            .split(text)
            .filter(|&s| s.is_empty() == false)
            .map(String::from)
            .collect()
    }
}

/// Get vector of columns to use from header row
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
            // Iterate through selector in vector of selectors
            for column_selector in &mut *column_selectors {
                if column_selector.stopped {
                    // Continue to next selector if a current selector's whole range has already
                    // been captured
                    continue;
                }
                // Keep track of whether column is contained by one selector's sequence to avoid
                // re-checking a column if already captured
                let mut in_sequence: bool = false;
                if (col_idx == column_selector.start_idx
                    && (column_selector.start_regex.as_str() == ".^"))
                    || column_selector.start_regex.is_match(column)
                {
                    // Sequence started
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
        // Return indexes of matched columns
        export_column_idxs
    }
}

/// Grab cells in a row by a list of given indeces
fn get_cells(row: &String,
             cells_to_select: &Vec<usize>,
             column_delimiter: &String
) -> Vec<String> {
    if cells_to_select.len() == 0 {
        // If no cells to select specified, return one element vector of the row
        vec![(*row).clone()]
    } else {
        // Iterate through cells in row and push ones with matching indeces to output vector
        let mut output: Vec<String> = Vec::new();
        for (cell_idx, cell) in split(row, column_delimiter).iter().enumerate() {
            if cells_to_select.contains(&cell_idx) {
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

    // Parse selectors
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
                // Continue to next selector if a current selector's whole range has already
                // been captured
                continue;
            }
            // Keep track of whether column is contained by one selector's sequence to avoid
            // re-checking a column if already captured
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
    // Iterate through results and print output
    for line in output {
        println!("{}", line.join("\t"));
    }
}
