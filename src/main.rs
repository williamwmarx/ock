use clap::Parser;
use std::process;

mod cli;
mod selector;
use selector::SelectorError;

include!("utils.rs");

/// Track selection state during iteration without mutating the selector
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionState {
    /// Current resolved start index for this iteration
    pub current_start_idx: usize,
    /// Current resolved end index for this iteration  
    pub current_end_idx: usize,
    /// Whether the selection has been stopped (for single item selections)
    pub stopped: bool,
}

#[cfg_attr(test, allow(dead_code))]
pub fn item_in_sequence_with_state(
    item_idx: usize, 
    item: &str, 
    selector: &selector::Selector, 
    state: &mut SelectionState,
    collection_length: usize
) -> bool {
    // Create a mutable copy for index resolution (temporary compatibility)
    let mut temp_selector = selector.clone();
    temp_selector.resolve_indices(collection_length);

    let mut in_sequence = false;
    
    // If a regex is provided as the only selector, just check against it
    if item_idx != temp_selector.resolved_start_idx
        && temp_selector.resolved_start_idx == temp_selector.resolved_end_idx
        && utils::regex_eq(&temp_selector.start_regex, &temp_selector.end_regex)
        && !utils::regex_is_default(&temp_selector.start_regex)
    {
        return temp_selector.start_regex.is_match(item);
    }
    
    if (item_idx == temp_selector.resolved_start_idx && utils::regex_is_default(&temp_selector.start_regex))
        || temp_selector.start_regex.is_match(item)
    {
        // Sequence started
        in_sequence = true;
        state.current_start_idx = item_idx;
        if (utils::regex_eq(&temp_selector.end_regex, &temp_selector.start_regex)
            && !utils::regex_is_default(&temp_selector.start_regex))
            || (temp_selector.resolved_end_idx == temp_selector.resolved_start_idx)
        {
            // Only one column selected
            state.stopped = true;
        }
    } else if state.current_start_idx != usize::MAX
        && ((item_idx == temp_selector.resolved_end_idx
            && item_idx >= state.current_start_idx
            && item_idx.saturating_sub(state.current_start_idx) % temp_selector.step == 0)
            || temp_selector.end_regex.is_match(item))
    {
        // Sequence end
        in_sequence = true;
        state.current_end_idx = item_idx;
    } else if item_idx > state.current_start_idx
        && item_idx < state.current_end_idx
        && item_idx.saturating_sub(state.current_start_idx) % temp_selector.step == 0
    {
        // Sequence middle
        in_sequence = true;
    }
    in_sequence
}

// Keep the old function for backward compatibility during transition
#[cfg_attr(test, allow(dead_code))]
pub fn item_in_sequence(item_idx: usize, item: &str, selector: &mut selector::Selector, collection_length: usize) -> bool {
    // Resolve indices if not already done
    selector.resolve_indices(collection_length);

    let mut in_sequence = false;
    if item_idx != selector.resolved_start_idx
        && selector.resolved_start_idx == selector.resolved_end_idx
        && utils::regex_eq(&selector.start_regex, &selector.end_regex)
        && !utils::regex_is_default(&selector.start_regex)
    {
        // If a regex is provided as the only selector, just check against it
        return selector.start_regex.is_match(item);
    }
    if (item_idx == selector.resolved_start_idx && utils::regex_is_default(&selector.start_regex))
        || selector.start_regex.is_match(item)
    {
        // Sequence started
        in_sequence = true;
        selector.resolved_start_idx = item_idx;
        if (utils::regex_eq(&selector.end_regex, &selector.start_regex)
            && !utils::regex_is_default(&selector.start_regex))
            || (selector.resolved_end_idx == selector.resolved_start_idx)
        {
            // Only one column selected
            selector.stopped = true;
        }
    } else if selector.resolved_start_idx != usize::MAX
        && ((item_idx == selector.resolved_end_idx
            && item_idx >= selector.resolved_start_idx
            && item_idx.saturating_sub(selector.resolved_start_idx) % selector.step == 0)
            || selector.end_regex.is_match(item))
    {
        // Sequence end
        in_sequence = true;
        selector.resolved_end_idx = item_idx;
    } else if item_idx > selector.resolved_start_idx
        && item_idx < selector.resolved_end_idx
        && item_idx.saturating_sub(selector.resolved_start_idx) % selector.step == 0
    {
        // Sequence middle
        in_sequence = true;
    }
    in_sequence
}

/// Get vector of columns to use from header row (immutable version)
#[cfg_attr(test, allow(dead_code))]
pub fn get_columns_immutable(
    index_row: &str,
    column_selectors: &[selector::Selector],
    column_delimiter: &str,
) -> Result<Vec<usize>, SelectorError> {
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
            for column_selector in column_selectors.iter() {
                let mut state = SelectionState {
                    current_start_idx: usize::MAX,
                    current_end_idx: usize::MAX,
                    stopped: false,
                };
                if item_in_sequence_with_state(col_idx, column, column_selector, &mut state, columns.len()) {
                    export_column_idxs.push(col_idx);
                }
            }
        }
        // Return indexes of matched columns
        Ok(export_column_idxs)
    }
}

/// Get vector of columns to use from header row (backward compatibility)
#[cfg_attr(test, allow(dead_code))]
pub fn get_columns(
    index_row: &str,
    column_selectors: &mut [selector::Selector],
    column_delimiter: &str,
) -> Result<Vec<usize>, SelectorError> {
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
                if item_in_sequence(col_idx, column, column_selector, columns.len()) {
                    export_column_idxs.push(col_idx);
                }
            }
        }
        // Return indexes of matched columns
        Ok(export_column_idxs)
    }
}

/// Get vector of columns and track which selectors matched (immutable version)
#[cfg_attr(test, allow(dead_code))]
pub fn get_columns_with_match_info_immutable(
    index_row: &str,
    column_selectors: &[selector::Selector],
    column_delimiter: &str,
    original_selectors_str: &str,
) -> Result<(Vec<usize>, Vec<String>), SelectorError> {
    if column_selectors.is_empty() {
        // Return empty vector when no column selectors provided (consistent with get_columns)
        return Ok((Vec::new(), Vec::new()));
    }

    let mut export_column_idxs: Vec<usize> = Vec::new();
    let mut matched_selectors: Vec<bool> = vec![false; column_selectors.len()];
    let columns = utils::split(index_row, column_delimiter)?;
    
    for (col_idx, column) in columns.iter().enumerate() {
        for (selector_idx, column_selector) in column_selectors.iter().enumerate() {
            let mut state = SelectionState {
                current_start_idx: usize::MAX,
                current_end_idx: usize::MAX,
                stopped: false,
            };
            if item_in_sequence_with_state(col_idx, column, column_selector, &mut state, columns.len()) {
                export_column_idxs.push(col_idx);
                matched_selectors[selector_idx] = true;
            }
        }
    }

    // Collect unmatched selector strings
    let original_parts: Vec<&str> = original_selectors_str.split(',').collect();
    let unmatched: Vec<String> = matched_selectors
        .iter()
        .enumerate()
        .filter_map(|(idx, &matched)| {
            if !matched && idx < original_parts.len() {
                Some(original_parts[idx].trim().to_string())
            } else {
                None
            }
        })
        .collect();

    Ok((export_column_idxs, unmatched))
}

/// Get vector of columns and track which selectors matched (backward compatibility)
#[cfg_attr(test, allow(dead_code))]
pub fn get_columns_with_match_info(
    index_row: &str,
    column_selectors: &mut [selector::Selector],
    column_delimiter: &str,
    original_selectors_str: &str,
) -> Result<(Vec<usize>, Vec<String>), SelectorError> {
    if column_selectors.is_empty() {
        // Return empty vector when no column selectors provided (consistent with get_columns)
        return Ok((Vec::new(), Vec::new()));
    }

    let mut export_column_idxs: Vec<usize> = Vec::new();
    let mut matched_selectors: Vec<bool> = vec![false; column_selectors.len()];
    let columns = utils::split(index_row, column_delimiter)?;
    
    for (col_idx, column) in columns.iter().enumerate() {
        for (selector_idx, column_selector) in column_selectors.iter_mut().enumerate() {
            if item_in_sequence(col_idx, column, column_selector, columns.len()) {
                export_column_idxs.push(col_idx);
                matched_selectors[selector_idx] = true;
            }
        }
    }

    // Collect unmatched selector strings
    let original_parts: Vec<&str> = original_selectors_str.split(',').collect();
    let unmatched: Vec<String> = matched_selectors
        .iter()
        .enumerate()
        .filter_map(|(idx, &matched)| {
            if !matched && idx < original_parts.len() {
                Some(original_parts[idx].trim().to_string())
            } else {
                None
            }
        })
        .collect();

    Ok((export_column_idxs, unmatched))
}

/// Grab cells in a row by a list of given indices.
///
/// When `cells_to_select` is empty, the entire row is returned only if
/// `select_full_row` is `true` (i.e., the caller provided no column selectors).
/// If indices are provided but none match, an empty vector is returned.
#[cfg_attr(test, allow(dead_code))]
pub fn get_cells(
    row: &str,
    cells_to_select: &[usize],
    column_delimiter: &str,
    select_full_row: bool,
) -> Result<Vec<String>, SelectorError> {
    if cells_to_select.is_empty() {
        if select_full_row {
            Ok(vec![row.to_string()])
        } else {
            Ok(Vec::new())
        }
    } else {
        // Iterate through cells in row and push ones with matching indices to output vector
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
    let select_full_row = args.columns.is_empty();

    // Parse selectors
    let row_selectors = match selector::parse_selectors(&args.rows) {
        Ok(selectors) => selectors,
        Err(e) => {
            eprintln!("Error parsing row selectors: {}", e);
            process::exit(1);
        }
    };
    let column_selectors = match selector::parse_selectors(&args.columns) {
        Ok(selectors) => selectors,
        Err(e) => {
            eprintln!("Error parsing column selectors: {}", e);
            process::exit(1);
        }
    };

    // Parse input data according to arguments
    let split_rows = match utils::split(&input, &args.row_delimiter) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Always process through column formatting pipeline
    let mut export_cols: Vec<usize> = Vec::new();
    let mut output: Vec<Vec<String>> = Vec::new();

    // Track selection state for each row selector
    let mut row_states: Vec<SelectionState> = row_selectors.iter().map(|_| SelectionState {
        current_start_idx: usize::MAX,
        current_end_idx: usize::MAX,
        stopped: false,
    }).collect();

    for (row_idx, row) in split_rows.iter().enumerate() {
        if row_idx == 0 {
            let (cols, unmatched) = match get_columns_with_match_info_immutable(
                row, 
                &column_selectors, 
                &args.column_delimiter, 
                &args.columns
            ) {
                Ok((cols, unmatched)) => (cols, unmatched),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            };
            export_cols = cols;
            
            // Only show warnings if specific column selectors were provided
            if !select_full_row {
                if export_cols.is_empty() {
                    eprintln!("Warning: No valid columns found for selection");
                } else if !unmatched.is_empty() {
                    eprintln!("Warning: Column selectors did not match any columns: {}", unmatched.join(", "));
                }
            }
        }
        for (selector_idx, row_selector) in row_selectors.iter().enumerate() {
            if item_in_sequence_with_state(row_idx, row, row_selector, &mut row_states[selector_idx], split_rows.len()) {
                let cells =
                    match get_cells(row, &export_cols, &args.column_delimiter, select_full_row)
                    {
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
