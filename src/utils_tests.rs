#[cfg(test)]
mod tests {
    use super::super::utils;
    use regex::Regex;

    #[test]
    fn test_regex_eq_identical() {
        let re1 = Regex::new(r"test").unwrap();
        let re2 = Regex::new(r"test").unwrap();
        assert!(utils::regex_eq(&re1, &re2));
    }

    #[test]
    fn test_regex_eq_different() {
        let re1 = Regex::new(r"test1").unwrap();
        let re2 = Regex::new(r"test2").unwrap();
        assert!(!utils::regex_eq(&re1, &re2));
    }

    #[test]
    fn test_regex_eq_complex_patterns() {
        let re1 = Regex::new(r"(?i).*pid.*").unwrap();
        let re2 = Regex::new(r"(?i).*pid.*").unwrap();
        assert!(utils::regex_eq(&re1, &re2));
        
        let re3 = Regex::new(r"(?i).*PID.*").unwrap();
        assert!(!utils::regex_eq(&re1, &re3)); // Different string representation
    }

    #[test]
    fn test_regex_is_default_true() {
        let re = Regex::new(r".^").unwrap();
        assert!(utils::regex_is_default(&re));
    }

    #[test]
    fn test_regex_is_default_false() {
        let re = Regex::new(r"test").unwrap();
        assert!(!utils::regex_is_default(&re));
        
        let re2 = Regex::new(r".*").unwrap();
        assert!(!utils::regex_is_default(&re2));
    }

    #[test]
    fn test_split_empty_delimiter() {
        let text = String::from("line1\nline2\nline3\n");
        let delimiter = String::from("");
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "line1");
        assert_eq!(result[1], "line2");
        assert_eq!(result[2], "line3");
    }

    #[test]
    fn test_split_empty_delimiter_with_empty_lines() {
        let text = String::from("line1\n\nline2\n\n\nline3");
        let delimiter = String::from("");
        let result = utils::split(&text, &delimiter).unwrap();
        
        // Empty lines should be filtered out
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "line1");
        assert_eq!(result[1], "line2");
        assert_eq!(result[2], "line3");
    }

    #[test]
    fn test_split_whitespace_delimiter() {
        let text = String::from("word1 word2  word3\tword4");
        let delimiter = String::from(r"\s+");
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "word1");
        assert_eq!(result[1], "word2");
        assert_eq!(result[2], "word3");
        assert_eq!(result[3], "word4");
    }

    #[test]
    fn test_split_comma_delimiter() {
        let text = String::from("apple,banana,cherry");
        let delimiter = String::from(",");
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "apple");
        assert_eq!(result[1], "banana");
        assert_eq!(result[2], "cherry");
    }

    #[test]
    fn test_split_pipe_delimiter() {
        let text = String::from("col1|col2|col3");
        let delimiter = String::from(r"\|");
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "col1");
        assert_eq!(result[1], "col2");
        assert_eq!(result[2], "col3");
    }

    #[test]
    fn test_split_tab_delimiter() {
        let text = String::from("field1\tfield2\tfield3");
        let delimiter = String::from(r"\t");
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "field1");
        assert_eq!(result[1], "field2");
        assert_eq!(result[2], "field3");
    }

    #[test]
    fn test_split_custom_delimiter() {
        let text = String::from("part1::part2::part3");
        let delimiter = String::from("::");
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "part1");
        assert_eq!(result[1], "part2");
        assert_eq!(result[2], "part3");
    }

    #[test]
    fn test_split_regex_delimiter() {
        let text = String::from("num1num2num3");
        let delimiter = String::from(r"\d+"); // Split on digits
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "num");
        assert_eq!(result[1], "num");
        assert_eq!(result[2], "num");
    }

    #[test]
    fn test_split_filters_empty_strings() {
        let text = String::from(",,a,b,,c,,");
        let delimiter = String::from(",");
        let result = utils::split(&text, &delimiter).unwrap();
        
        // Empty strings should be filtered out
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a");
        assert_eq!(result[1], "b");
        assert_eq!(result[2], "c");
    }

    #[test]
    fn test_split_complex_multiline() {
        let text = String::from("USER     PID   %CPU  %MEM    VSZ   RSS\nroot       1    0.0   0.0  12345  6789\nuser     123    1.5   2.3  98765  4321");
        let delimiter = String::from(r"\n");
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 3);
        assert!(result[0].starts_with("USER"));
        assert!(result[1].starts_with("root"));
        assert!(result[2].starts_with("user"));
    }

    #[test]
    fn test_split_edge_cases() {
        // Empty text
        let text = String::from("");
        let delimiter = String::from(",");
        let result = utils::split(&text, &delimiter).unwrap();
        assert_eq!(result.len(), 0);
        
        // Text with no delimiters
        let text = String::from("singleword");
        let delimiter = String::from(",");
        let result = utils::split(&text, &delimiter).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "singleword");
        
        // Text that is just delimiters
        let text = String::from(",,,");
        let delimiter = String::from(",");
        let result = utils::split(&text, &delimiter).unwrap();
        assert_eq!(result.len(), 0); // All empty strings filtered out
    }

    #[test]
    fn test_split_preserves_internal_spaces() {
        let text = String::from("hello world,foo bar,baz qux");
        let delimiter = String::from(",");
        let result = utils::split(&text, &delimiter).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "hello world");
        assert_eq!(result[1], "foo bar");
        assert_eq!(result[2], "baz qux");
    }

    #[test]
    fn test_split_default_whitespace_behavior() {
        let text = String::from("word1 word2  word3\t\tword4\n");
        let delimiter = String::from(r"\s");
        let result = utils::split(&text, &delimiter).unwrap();
        
        // Should split on any whitespace
        assert!(result.len() >= 4);
        assert!(result.contains(&String::from("word1")));
        assert!(result.contains(&String::from("word2")));
        assert!(result.contains(&String::from("word3")));
        assert!(result.contains(&String::from("word4")));
    }
}