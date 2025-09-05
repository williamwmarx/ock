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

/// Format output with column alignment for pretty printing
#[cfg_attr(test, allow(dead_code))]
pub fn format_columns(output: &[Vec<String>]) -> Vec<String> {
    if output.is_empty() {
        return Vec::new();
    }

    // Calculate max width for each column
    let mut col_widths: Vec<usize> = Vec::new();
    for row in output {
        for (col_idx, cell) in row.iter().enumerate() {
            if col_idx >= col_widths.len() {
                col_widths.push(0);
            }
            col_widths[col_idx] = col_widths[col_idx].max(cell.len());
        }
    }

    // Format output with alignment
    let mut result: Vec<String> = Vec::new();
    for row in output {
        let mut formatted_row = String::new();
        for (col_idx, cell) in row.iter().enumerate() {
            if col_idx == row.len() - 1 {
                formatted_row.push_str(cell);
            } else {
                formatted_row.push_str(&format!("{:width$} ", cell, width = col_widths[col_idx]));
            }
        }
        result.push(formatted_row);
    }
    result
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

    // Format and print results
    let formatted_output = format_columns(&output);
    for line in formatted_output {
        println!("{}", line);
    }
}

#[cfg(test)]
#[path = "utils_tests.rs"]
mod utils_tests;

#[cfg(test)]
#[path = "main_tests.rs"]
mod main_tests;
