#[cfg(test)]
mod tests {
    use super::super::*;
    use regex::Regex;

    #[test]
    fn test_selector_default() {
        let selector = Selector::default();
        assert_eq!(selector.start_idx, 0);
        assert_eq!(selector.end_idx, std::usize::MAX);
        assert_eq!(selector.step, 1);
        assert_eq!(selector.stopped, false);
        assert_eq!(selector.start_regex.as_str(), ".^");
        assert_eq!(selector.end_regex.as_str(), ".^");
    }

    #[test]
    fn test_selector_partial_eq() {
        let selector1 = Selector::default();
        let selector2 = Selector::default();
        assert_eq!(selector1, selector2);

        let mut selector3 = Selector::default();
        selector3.start_idx = 5;
        assert_ne!(selector1, selector3);

        let mut selector4 = Selector::default();
        selector4.start_regex = Regex::new(r"test").unwrap();
        assert_ne!(selector1, selector4);
    }

    #[test]
    fn test_parse_selectors_single_index() {
        let selectors = parse_selectors(&String::from("5"));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, 4); // 5-1
        assert_eq!(selectors[0].end_idx, 4);
        assert_eq!(selectors[0].step, 1);
    }

    #[test]
    fn test_parse_selectors_range() {
        let selectors = parse_selectors(&String::from("2:10"));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, 1); // 2-1
        assert_eq!(selectors[0].end_idx, 9);   // 10-1
        assert_eq!(selectors[0].step, 1);
    }

    #[test]
    fn test_parse_selectors_range_with_step() {
        let selectors = parse_selectors(&String::from("1:10:2"));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, 0); // 1-1 (correct: convert to 0-based index)
        assert_eq!(selectors[0].end_idx, 9);   // 10-1 (correct: convert to 0-based index)
        assert_eq!(selectors[0].step, 2);      // BUG: Expected 2, but gets 1 due to incorrect subtraction
    }

    #[test]
    fn test_parse_selectors_multiple() {
        let selectors = parse_selectors(&String::from("1,5,10"));
        assert_eq!(selectors.len(), 3);
        
        assert_eq!(selectors[0].start_idx, 0);
        assert_eq!(selectors[0].end_idx, 0);
        
        assert_eq!(selectors[1].start_idx, 4);
        assert_eq!(selectors[1].end_idx, 4);
        
        assert_eq!(selectors[2].start_idx, 9);
        assert_eq!(selectors[2].end_idx, 9);
    }

    #[test]
    fn test_parse_selectors_regex() {
        let selectors = parse_selectors(&String::from("pid"));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, usize::MAX);
        assert!(selectors[0].start_regex.is_match("PID"));
        assert!(selectors[0].start_regex.is_match("pid"));
        assert!(selectors[0].start_regex.is_match("some_pid_value"));
        assert_eq!(selectors[0].start_regex.as_str(), "(?i).*pid.*");
        assert_eq!(selectors[0].end_regex.as_str(), "(?i).*pid.*");
    }

    #[test]
    fn test_parse_selectors_regex_range() {
        let selectors = parse_selectors(&String::from("start:end"));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, usize::MAX);
        assert!(selectors[0].start_regex.is_match("START"));
        assert!(selectors[0].end_regex.is_match("END"));
        assert_eq!(selectors[0].start_regex.as_str(), "(?i).*start.*");
        assert_eq!(selectors[0].end_regex.as_str(), "(?i).*end.*");
    }

    #[test]
    fn test_parse_selectors_mixed_regex_and_index() {
        let selectors = parse_selectors(&String::from("pid,5"));
        assert_eq!(selectors.len(), 2);
        
        // First selector is a regex
        assert_eq!(selectors[0].start_idx, usize::MAX);
        assert!(selectors[0].start_regex.is_match("PID"));
        
        // Second selector is an index
        assert_eq!(selectors[1].start_idx, 4);
        assert_eq!(selectors[1].end_idx, 4);
    }

    #[test]
    fn test_parse_selectors_empty_components() {
        let selectors = parse_selectors(&String::from(":10"));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, 0); // Default start
        assert_eq!(selectors[0].end_idx, 9);   // 10-1
        
        let selectors = parse_selectors(&String::from("5:"));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, 4);         // 5-1
        assert_eq!(selectors[0].end_idx, usize::MAX); // Default end
        
        let selectors = parse_selectors(&String::from("::2"));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, 0);         // Default start
        assert_eq!(selectors[0].end_idx, usize::MAX);  // Default end
        assert_eq!(selectors[0].step, 2);              // BUG: Expected 2, but gets 1
    }

    #[test]
    fn test_parse_selectors_complex_multiple() {
        let selectors = parse_selectors(&String::from("1:5,pid,10:20:2,name"));
        assert_eq!(selectors.len(), 4);
        
        // First: range 1:5
        assert_eq!(selectors[0].start_idx, 0);
        assert_eq!(selectors[0].end_idx, 4);
        assert_eq!(selectors[0].step, 1);
        
        // Second: regex "pid"
        assert!(selectors[1].start_regex.is_match("PID"));
        
        // Third: range with step 10:20:2
        assert_eq!(selectors[2].start_idx, 9);
        assert_eq!(selectors[2].end_idx, 19);
        assert_eq!(selectors[2].step, 2); // BUG: Expected 2, but gets 1
        
        // Fourth: regex "name"
        assert!(selectors[3].start_regex.is_match("NAME"));
    }

    #[test]
    #[should_panic(expected = "Step size must be an integer")]
    fn test_parse_selectors_non_integer_step() {
        parse_selectors(&String::from("1:10:abc"));
    }

    #[test]
    #[should_panic(expected = "A selector cannot be more than three components long")]
    fn test_parse_selectors_too_many_components() {
        parse_selectors(&String::from("1:2:3:4"));
    }

    #[test]
    fn test_parse_selectors_edge_cases() {
        // Test with 1 as index (should become 0)
        let selectors = parse_selectors(&String::from("1"));
        assert_eq!(selectors[0].start_idx, 0);
        assert_eq!(selectors[0].end_idx, 0);
        
        // Test empty string
        let selectors = parse_selectors(&String::from(""));
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].start_idx, 0);
        assert_eq!(selectors[0].end_idx, usize::MAX);
        
        // Test multiple commas
        let selectors = parse_selectors(&String::from("1,,3"));
        assert_eq!(selectors.len(), 3);
        assert_eq!(selectors[0].start_idx, 0);
        assert_eq!(selectors[1].start_idx, 0); // Empty selector gets default
        assert_eq!(selectors[2].start_idx, 2);
    }

    #[test]
    fn test_parse_selectors_case_insensitive_regex() {
        let selectors = parse_selectors(&String::from("PID"));
        assert!(selectors[0].start_regex.is_match("pid"));
        assert!(selectors[0].start_regex.is_match("PID"));
        assert!(selectors[0].start_regex.is_match("Pid"));
        assert!(selectors[0].start_regex.is_match("pId"));
    }

    #[test]
    fn test_parse_selectors_partial_match_regex() {
        let selectors = parse_selectors(&String::from("user"));
        assert!(selectors[0].start_regex.is_match("user"));
        assert!(selectors[0].start_regex.is_match("username"));
        assert!(selectors[0].start_regex.is_match("superuser"));
        assert!(selectors[0].start_regex.is_match("multiuser"));
    }
}