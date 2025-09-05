use clap::Parser;
use std::process;

mod cli;
mod selector;

include!("utils.rs");

#[cfg_attr(test, allow(dead_code))]
pub fn item_in_sequence(item_idx: usize, item: &str, selector: &mut selector::Selector) -> bool {
    let mut in_sequence = false;
    if item_idx != selector.start_idx
        && selector.start_idx == selector.end_idx
        && utils::regex_eq(&selector.start_regex, &selector.end_regex)
        && !utils::regex_is_default(&selector.start_regex)
    {
        // If a regex is provided as the only selector, just check against it
        return selector.start_regex.is_match(item);
    }
    if (item_idx == selector.start_idx && utils::regex_is_default(&selector.start_regex))
        || selector.start_regex.is_match(item)
    {
        // Sequence started
        in_sequence = true;
        selector.start_idx = item_idx;
        if (utils::regex_eq(&selector.end_regex, &selector.start_regex)
            && !utils::regex_is_default(&selector.start_regex))
            || (selector.end_idx == selector.start_idx)
        {
            // Only one column selected
            selector.stopped = true;
        }
    } else if selector.start_idx != usize::MAX
        && ((item_idx == selector.end_idx
            && item_idx >= selector.start_idx
            && (item_idx - selector.start_idx) % selector.step == 0)
            || selector.end_regex.is_match(item))
    {
        // Sequence end
        in_sequence = true;
        selector.end_idx = item_idx;
    } else if item_idx > selector.start_idx
        && item_idx < selector.end_idx
        && (item_idx - selector.start_idx) % selector.step == 0
    {
        // Sequence middle
        in_sequence = true;
    }
    in_sequence
}

/// Get vector of columns to use from header row
#[cfg_attr(test, allow(dead_code))]
pub fn get_columns(
    index_row: &str,
    column_selectors: &mut [selector::Selector],
    column_delimiter: &str,
) -> Result<Vec<usize>, String> {
    if column_selectors.is_empty() {
        // Return blank vector if no column selectors present
        Ok(Vec::new())
    } else {
        // Return a vector of column indices to export
        let mut export_column_idxs: Vec<usize> = Vec::new();
        // Iterate through columns in first row
        let columns = utils::split(index_row, column_delimiter)?;
        for (col_idx, column) in columns.iter().enumerate() {
            // Iterate through selector in vector of selectors
            for column_selector in column_selectors.iter_mut() {
                if item_in_sequence(col_idx, column, column_selector) {
                    export_column_idxs.push(col_idx);
                }
            }
        }
        // Return indexes of matched columns
        Ok(export_column_idxs)
    }
}

/// Grab cells in a row by a list of given indeces
#[cfg_attr(test, allow(dead_code))]
pub fn get_cells(row: &str, cells_to_select: &[usize], column_delimiter: &str) -> Result<Vec<String>, String> {
    if cells_to_select.is_empty() {
        // If no cells to select specified, return one element vector of the row
        Ok(vec![row.to_string()])
    } else {
        // Iterate through cells in row and push ones with matching indeces to output vector
        let mut output: Vec<String> = Vec::new();
        let cells = utils::split(row, column_delimiter)?;
        for (cell_idx, cell) in cells.iter().enumerate() {
            if cells_to_select.contains(&cell_idx) {
                output.push(cell.clone());
            }
        }
        Ok(output)
    }
}

fn main() {
    // Parse arguments
    let args = cli::Args::parse();
    let input = cli::parse_input(&args.input);

    // Parse selectors
    let mut row_selectors = match selector::parse_selectors(&args.rows) {
        Ok(selectors) => selectors,
        Err(e) => {
            eprintln!("Error parsing row selectors: {}", e);
            process::exit(1);
        }
    };
    let mut column_selectors = match selector::parse_selectors(&args.columns) {
        Ok(selectors) => selectors,
        Err(e) => {
            eprintln!("Error parsing column selectors: {}", e);
            process::exit(1);
        }
    };

    // Parse input data according to arguments
    let mut export_cols: Vec<usize> = Vec::new();
    let mut output: Vec<Vec<String>> = Vec::new();
    let split_rows = match utils::split(&input, &args.row_delimiter) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    for (row_idx, row) in split_rows.iter().enumerate() {
        if row_idx == 0 {
            export_cols = match get_columns(row, &mut column_selectors, &args.column_delimiter) {
                Ok(cols) => cols,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            };
        }
        for row_selector in row_selectors.iter_mut() {
            if item_in_sequence(row_idx, row, row_selector) {
                let cells = match get_cells(row, &export_cols, &args.column_delimiter) {
                    Ok(cells) => cells,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                };
                output.push(cells);
            }
        }
    }

    // Iterate through results and find max length of each column for pretty printing
    if output.is_empty() {
        return; // No output to print
    }
    let mut max_column_lengths: Vec<usize> = output[0].iter().map(|s| s.len()).collect();
    for row in &output {
        for (idx, cell) in row.iter().enumerate() {
            let cell_length = cell.len();
            if cell_length > max_column_lengths[idx] {
                max_column_lengths[idx] = cell_length;
            }
        }
    }

    // Print results to screen
    for row in &output {
        let mut formatted_row: String = String::new();
        for (idx, cell) in row.iter().enumerate() {
            let formatted_cell = format!("{:width$}", cell, width = max_column_lengths[idx] + 2);
            formatted_row.push_str(&formatted_cell);
        }
        println!("{}", formatted_row)
    }
}

#[cfg(test)]
#[path = "utils_tests.rs"]
mod utils_tests;

#[cfg(test)]
#[path = "main_tests.rs"]
mod main_tests;
