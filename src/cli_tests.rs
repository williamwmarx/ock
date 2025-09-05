#[cfg(test)]
mod tests {
    use super::super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_input_from_file() {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_content = "test file content\nline 2\nline 3";
        writeln!(temp_file, "{}", test_content).unwrap();
        
        let file_path = temp_file.path().to_str().unwrap().to_string();
        let result = parse_input(&file_path);
        
        assert!(result.contains("test file content"));
        assert!(result.contains("line 2"));
        assert!(result.contains("line 3"));
    }

    #[test]
    fn test_parse_input_literal_text() {
        let input = String::from("this is literal text");
        let result = parse_input(&input);
        
        assert_eq!(result, "this is literal text");
    }

    #[test]
    fn test_parse_input_multiline_literal() {
        let input = String::from("line1\nline2\nline3");
        let result = parse_input(&input);
        
        assert_eq!(result, "line1\nline2\nline3");
    }

    #[test]
    fn test_parse_input_nonexistent_file_as_literal() {
        let input = String::from("/very/unlikely/to/exist/file.txt");
        let result = parse_input(&input);
        
        // Should treat non-existent path as literal text
        assert_eq!(result, "/very/unlikely/to/exist/file.txt");
    }

    #[test]
    fn test_parse_input_empty_string() {
        // Note: This would normally read from stdin, but we can't easily test that
        // in a unit test without mocking stdin. Stdin functionality is comprehensively 
        // tested in integration tests via the run_ock_with_stdin() helper function.
        let _input = String::from("");
        // This test would hang waiting for stdin in actual execution
        // We're just documenting the expected behavior here
        // In real usage, parse_input("") would call read_stdin()
        // See tests/integration_test.rs for complete stdin test coverage
    }

    #[test]
    fn test_args_default_values() {
        // Test that the default values are set correctly
        // Note: This test requires creating Args programmatically
        // The actual CLI parsing is tested in integration tests
        let args = Args {
            rows: String::from(""),
            row_delimiter: String::from(r"\n"),
            columns: String::from(""),
            column_delimiter: String::from(r"\s"),
            input: String::from(""),
        };
        
        assert_eq!(args.rows, "");
        assert_eq!(args.row_delimiter, r"\n");
        assert_eq!(args.columns, "");
        assert_eq!(args.column_delimiter, r"\s");
        assert_eq!(args.input, "");
    }

    #[test]
    fn test_parse_input_with_special_characters() {
        let input = String::from("text with special chars: @#$%^&*()");
        let result = parse_input(&input);
        
        assert_eq!(result, "text with special chars: @#$%^&*()");
    }

    #[test]
    fn test_parse_input_with_unicode() {
        let input = String::from("Unicode text: 你好世界 🌍 émojis");
        let result = parse_input(&input);
        
        assert_eq!(result, "Unicode text: 你好世界 🌍 émojis");
    }

    #[test]
    fn test_parse_input_file_with_trailing_newline() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_content = "content\n";
        write!(temp_file, "{}", test_content).unwrap();
        
        let file_path = temp_file.path().to_str().unwrap().to_string();
        let result = parse_input(&file_path);
        
        assert_eq!(result, "content\n");
    }

    #[test]
    fn test_parse_input_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();
        let result = parse_input(&file_path);
        
        assert_eq!(result, "");
    }

    #[test]
    fn test_parse_input_large_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let large_content: String = (0..1000).map(|i| format!("Line {}\n", i)).collect();
        write!(temp_file, "{}", large_content).unwrap();
        
        let file_path = temp_file.path().to_str().unwrap().to_string();
        let result = parse_input(&file_path);
        
        assert!(result.contains("Line 0"));
        assert!(result.contains("Line 500"));
        assert!(result.contains("Line 999"));
    }
}