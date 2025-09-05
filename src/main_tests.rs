#[cfg(test)]
mod tests {
    use crate::selector::{Selector, parse_selectors};
    use crate::{item_in_sequence, get_columns, get_cells, format_columns};
    use regex::Regex;

    #[test]
    fn test_item_in_sequence_single_index() {
        let mut selector = Selector::default();
        selector.start_idx = 2;
        selector.end_idx = 2;

        let item = String::from("test");
        assert!(!item_in_sequence(0, &item, &mut selector));
        assert!(!item_in_sequence(1, &item, &mut selector));
        assert!(item_in_sequence(2, &item, &mut selector));
        assert!(!item_in_sequence(3, &item, &mut selector));
    }

    #[test]
    fn test_item_in_sequence_range() {
        let mut selector = Selector::default();
        selector.start_idx = 2;
        selector.end_idx = 5;

        let item = String::from("test");
        assert!(!item_in_sequence(0, &item, &mut selector));
        assert!(!item_in_sequence(1, &item, &mut selector));
        assert!(item_in_sequence(2, &item, &mut selector));
        assert!(item_in_sequence(3, &item, &mut selector));
        assert!(item_in_sequence(4, &item, &mut selector));
        assert!(item_in_sequence(5, &item, &mut selector));
        assert!(!item_in_sequence(6, &item, &mut selector));
    }

    #[test]
    fn test_item_in_sequence_with_step() {
        let mut selector = Selector::default();
        selector.start_idx = 0;
        selector.end_idx = 10;
        selector.step = 2;

        let item = String::from("test");
        assert!(item_in_sequence(0, &item, &mut selector));
        assert!(!item_in_sequence(1, &item, &mut selector));
        assert!(item_in_sequence(2, &item, &mut selector));
        assert!(!item_in_sequence(3, &item, &mut selector));
        assert!(item_in_sequence(4, &item, &mut selector));
        assert!(!item_in_sequence(5, &item, &mut selector));
        assert!(item_in_sequence(6, &item, &mut selector));
    }

    #[test]
    fn test_item_in_sequence_regex_single() {
        let mut selector = Selector::default();
        selector.start_regex = Regex::new(r"(?i).*pid.*").unwrap();
        selector.end_regex = Regex::new(r"(?i).*pid.*").unwrap();
        selector.start_idx = usize::MAX;
        selector.end_idx = usize::MAX;

        let pid_item = String::from("PID");
        let user_item = String::from("USER");
        let process_pid = String::from("process_pid");

        assert!(item_in_sequence(0, &pid_item, &mut selector));
        assert!(!item_in_sequence(1, &user_item, &mut selector));
        assert!(item_in_sequence(2, &process_pid, &mut selector));
    }

    #[test]
    fn test_item_in_sequence_regex_range() {
        let mut selector = Selector::default();
        selector.start_regex = Regex::new(r"(?i).*start.*").unwrap();
        selector.end_regex = Regex::new(r"(?i).*end.*").unwrap();

        let start = String::from("START");
        let middle = String::from("MIDDLE");
        let end = String::from("END");

        // Before match
        assert!(!item_in_sequence(0, &middle, &mut selector));

        // Start match
        assert!(item_in_sequence(1, &start, &mut selector));
        assert_eq!(selector.start_idx, 1);

        // Middle items (after start has been found)
        assert!(item_in_sequence(2, &middle, &mut selector));

        // End match
        assert!(item_in_sequence(3, &end, &mut selector));
    }

    #[test]
    fn test_item_in_sequence_stopped() {
        let mut selector = Selector::default();
        selector.start_idx = 2;
        selector.end_idx = 2;

        let item = String::from("test");
        assert!(item_in_sequence(2, &item, &mut selector));
        assert!(selector.stopped); // Should be stopped after single selection
    }

    #[test]
    fn test_get_columns_no_selectors() {
        let row = String::from("col1 col2 col3");
        let mut selectors: Vec<Selector> = Vec::new();
        let delimiter = String::from(r"\s");

        let result = get_columns(&row, &mut selectors, &delimiter);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_columns_single_index() {
        let row = String::from("col1 col2 col3");
        let mut selectors = parse_selectors(&String::from("2")).unwrap();
        let delimiter = String::from(r"\s");

        let result = get_columns(&row, &mut selectors, &delimiter);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 1); // Column 2 is index 1
    }

    #[test]
    fn test_get_columns_multiple_indices() {
        let row = String::from("col1 col2 col3 col4");
        let mut selectors = parse_selectors(&String::from("1,3")).unwrap();
        let delimiter = String::from(r"\s");

        let result = get_columns(&row, &mut selectors, &delimiter);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 0); // Column 1 is index 0
        assert_eq!(result[1], 2); // Column 3 is index 2
    }

    #[test]
    fn test_get_columns_range() {
        let row = String::from("col1 col2 col3 col4");
        let mut selectors = parse_selectors(&String::from("2:4")).unwrap();
        let delimiter = String::from(r"\s");

        let result = get_columns(&row, &mut selectors, &delimiter);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 1); // Column 2
        assert_eq!(result[1], 2); // Column 3
        assert_eq!(result[2], 3); // Column 4
    }

    #[test]
    fn test_get_columns_regex() {
        let row = String::from("USER PID COMMAND");
        let mut selectors = parse_selectors(&String::from("pid")).unwrap();
        let delimiter = String::from(r"\s");

        let result = get_columns(&row, &mut selectors, &delimiter);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 1); // PID is index 1
    }

    #[test]
    fn test_get_columns_mixed() {
        let row = String::from("USER PID %CPU %MEM COMMAND");
        let mut selectors = parse_selectors(&String::from("1,pid,%mem")).unwrap();
        let delimiter = String::from(r"\s");

        let result = get_columns(&row, &mut selectors, &delimiter);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 0); // USER is index 0
        assert_eq!(result[1], 1); // PID is index 1
        assert_eq!(result[2], 3); // %MEM is index 3
    }

    #[test]
    fn test_get_columns_custom_delimiter() {
        let row = String::from("col1,col2,col3,col4");
        let mut selectors = parse_selectors(&String::from("2:3")).unwrap();
        let delimiter = String::from(",");

        let result = get_columns(&row, &mut selectors, &delimiter);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 1); // col2
        assert_eq!(result[1], 2); // col3
    }

    #[test]
    fn test_get_cells_no_selection() {
        let row = String::from("cell1 cell2 cell3");
        let cells_to_select: Vec<usize> = Vec::new();
        let delimiter = String::from(r"\s");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "cell1 cell2 cell3");
    }

    #[test]
    fn test_get_cells_single_cell() {
        let row = String::from("cell1 cell2 cell3");
        let cells_to_select = vec![1];
        let delimiter = String::from(r"\s");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "cell2");
    }

    #[test]
    fn test_get_cells_multiple_cells() {
        let row = String::from("cell1 cell2 cell3 cell4");
        let cells_to_select = vec![0, 2, 3];
        let delimiter = String::from(r"\s");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "cell1");
        assert_eq!(result[1], "cell3");
        assert_eq!(result[2], "cell4");
    }

    #[test]
    fn test_get_cells_out_of_order_indices() {
        let row = String::from("A B C D");
        let cells_to_select = vec![3, 1, 0];
        let delimiter = String::from(r"\s");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "A");
        assert_eq!(result[1], "B");
        assert_eq!(result[2], "D");
    }

    #[test]
    fn test_get_cells_custom_delimiter() {
        let row = String::from("a,b,c,d,e");
        let cells_to_select = vec![1, 3];
        let delimiter = String::from(",");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "b");
        assert_eq!(result[1], "d");
    }

    #[test]
    fn test_get_cells_tab_delimiter() {
        let row = String::from("field1\tfield2\tfield3");
        let cells_to_select = vec![0, 2];
        let delimiter = String::from(r"\t");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "field1");
        assert_eq!(result[1], "field3");
    }

    #[test]
    fn test_get_cells_empty_cells() {
        let row = String::from("a,,c");
        let cells_to_select = vec![0, 1];
        let delimiter = String::from(",");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "a");
        assert_eq!(result[1], "c"); // Empty cell is filtered out
    }

    #[test]
    fn test_get_cells_index_out_of_bounds() {
        let row = String::from("a b c");
        let cells_to_select = vec![0, 5, 10]; // Indices beyond the row length
        let delimiter = String::from(r"\s");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "a"); // Only the valid index is included
    }

    #[test]
    fn test_get_cells_preserves_spaces() {
        let row = String::from("hello world,foo bar,baz qux");
        let cells_to_select = vec![0, 2];
        let delimiter = String::from(",");

        let result = get_cells(&row, &cells_to_select, &delimiter);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "hello world");
        assert_eq!(result[1], "baz qux");
    }

    // Column alignment tests
    #[test]
    fn test_column_alignment_empty_output() {
        let output: Vec<Vec<String>> = Vec::new();
        let result = format_columns(&output);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_column_alignment_single_column() {
        let output = vec![
            vec!["a".to_string()],
            vec!["bb".to_string()],
            vec!["ccc".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a");
        assert_eq!(result[1], "bb");
        assert_eq!(result[2], "ccc");
    }

    #[test]
    fn test_column_alignment_basic() {
        let output = vec![
            vec!["a".to_string(), "bb".to_string(), "ccc".to_string()],
            vec!["1111".to_string(), "2".to_string(), "33".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "a    bb ccc");
        assert_eq!(result[1], "1111 2  33");
    }

    #[test]
    fn test_column_alignment_varying_widths() {
        let output = vec![
            vec!["short".to_string(), "medium".to_string(), "very_long_content".to_string()],
            vec!["x".to_string(), "y".to_string(), "z".to_string()],
            vec!["longer".to_string(), "text".to_string(), "here".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "short  medium very_long_content");
        assert_eq!(result[1], "x      y      z");
        assert_eq!(result[2], "longer text   here");
    }

    #[test]
    fn test_column_alignment_empty_cells() {
        let output = vec![
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec!["".to_string(), "longer".to_string(), "".to_string()],
            vec!["d".to_string(), "".to_string(), "f".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a longer c");
        assert_eq!(result[1], "  longer  ");
        assert_eq!(result[2], "d        f");
    }

    #[test]
    fn test_column_alignment_different_row_lengths() {
        // Test rows with different numbers of columns
        let output = vec![
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec!["1".to_string()],
            vec!["d".to_string(), "e".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a b c");
        assert_eq!(result[1], "1");
        assert_eq!(result[2], "d e");
    }

    #[test]
    fn test_column_alignment_unicode() {
        let output = vec![
            vec!["短".to_string(), "longer".to_string()],
            vec!["很长的文本".to_string(), "x".to_string()],
            vec!["中".to_string(), "medium".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 3);
        // Note: This test verifies that the function handles unicode characters
        // The actual alignment might not be perfect for display due to character width differences
        assert_eq!(result[0], "短       longer");
        assert_eq!(result[1], "很长的文本 x");
        assert_eq!(result[2], "中       medium");
    }

    #[test]
    fn test_column_alignment_numbers_text_special() {
        let output = vec![
            vec!["123".to_string(), "text".to_string(), "@#$%".to_string()],
            vec!["45".to_string(), "longer_text".to_string(), "!".to_string()],
            vec!["6789".to_string(), "a".to_string(), "~~".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "123  text        @#$%");
        assert_eq!(result[1], "45   longer_text !");
        assert_eq!(result[2], "6789 a           ~~");
    }

    #[test]
    fn test_column_alignment_single_row() {
        let output = vec![
            vec!["single".to_string(), "row".to_string(), "test".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "single row test");
    }

    #[test]
    fn test_column_alignment_whitespace_preservation() {
        let output = vec![
            vec!["has spaces".to_string(), "no_spaces".to_string()],
            vec!["tabs\there".to_string(), "normal".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "has spaces no_spaces");
        assert_eq!(result[1], "tabs\there  normal");
    }

    #[test]
    fn test_column_alignment_very_long_cells() {
        let long_string = "this_is_a_very_long_string_that_should_affect_column_width";
        let output = vec![
            vec!["short".to_string(), long_string.to_string()],
            vec!["x".to_string(), "y".to_string()],
        ];
        let result = format_columns(&output);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], format!("short {}", long_string));
        assert_eq!(result[1], format!("x     {}", "y"));
    }
}
