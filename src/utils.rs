mod utils {
    use regex::Regex;

    /// Test is two regex expressions are equal
    /// This needs to be done as there's no PartialEq provided by regex::Regex
    #[allow(dead_code)]
    pub fn regex_eq(re1: &Regex, re2: &Regex) -> bool {
        // Convert both regexes to strings and check their equality
        re1.as_str() == re2.as_str()
    }

    /// Regex is default, which is the impossible regex ".^"
    #[allow(dead_code)]
    pub fn regex_is_default(re: &Regex) -> bool {
        re.as_str() == ".^"
    }

    /// Split given text by a delimiter, returning a vector of Strings
    #[allow(dead_code)]
    pub fn split(text: &str, delimiter: &str) -> Vec<String> {
        if delimiter.is_empty() {
            // Split by lines if empty delmiter passed. This should be faster than regex split
            text.lines()
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        } else {
            // Split by regex
            Regex::new(delimiter)
                .unwrap()
                .split(text)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        }
    }
}
