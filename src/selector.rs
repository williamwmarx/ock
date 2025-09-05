use regex::Regex;
use std::fmt;
include!("utils.rs");

#[derive(Debug)]
pub enum SelectorError {
    InvalidRegex { pattern: String, source: regex::Error },
}

impl fmt::Display for SelectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SelectorError::InvalidRegex { pattern, source } => {
                write!(f, "Invalid regex pattern '{}': {}", pattern, source)
            }
        }
    }
}

impl std::error::Error for SelectorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SelectorError::InvalidRegex { source, .. } => Some(source),
        }
    }
}

/// Keep track of user column and row selections
#[derive(Debug)]
pub struct Selector {
    /// Index of first row to grab (start of range)
    pub start_idx: usize,

    /// Regex of first to grab (start of range)
    pub start_regex: regex::Regex,

    /// Index of last row to grab (end of range)
    pub end_idx: usize,

    /// Regex of last row to grab (end of range)
    pub end_regex: regex::Regex,

    /// Step size between start and end of range
    pub step: usize,

    /// Keep track of when to stop adding rows from range to output
    pub stopped: bool,
}

impl Selector {
    /// Create a new default selector
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(Selector)` with default values or `Err(SelectorError)` if the default regex cannot be compiled.
    /// This should never fail in practice since we use a known-good regex pattern.
    pub fn new() -> Result<Selector, SelectorError> {
        let default_regex = r".^";
        let start_regex = Regex::new(default_regex)
            .map_err(|e| SelectorError::InvalidRegex { 
                pattern: default_regex.to_string(), 
                source: e 
            })?;
        let end_regex = Regex::new(default_regex)
            .map_err(|e| SelectorError::InvalidRegex { 
                pattern: default_regex.to_string(), 
                source: e 
            })?;
            
        Ok(Selector {
            start_idx: 0,
            start_regex,
            end_idx: usize::MAX,
            end_regex,
            step: 1,
            stopped: false,
        })
    }
}

impl Default for Selector {
    /// Defaults to implement a new selector without defining each field individually
    /// 
    /// # Panics
    /// 
    /// This will panic if the default regex pattern fails to compile, which should never happen.
    /// For error handling, use `Selector::new()` instead.
    fn default() -> Selector {
        Selector::new().expect("Default selector regex should always compile")
    }
}

impl PartialEq for Selector {
    /// Enable checking the equality of two Selector structs
    /// We do this by simply ensuring each field in the structs are equal
    /// While this seems straight forward, it's necessary as `regex::Regex` does not have a
    /// PartialEq implemented by default.
    fn eq(&self, other: &Self) -> bool {
        self.start_idx == other.start_idx
            && utils::regex_eq(&self.start_regex, &other.start_regex)
            && self.end_idx == other.end_idx
            && utils::regex_eq(&self.end_regex, &other.end_regex)
            && self.step == other.step
            && self.stopped == other.stopped
    }
}

/// Parse either row or column selectors, turning Python-like list slicing syntax into vector of
/// Selector structs
/// 
/// # Errors
/// 
/// Returns `SelectorError::InvalidRegex` if any regex pattern fails to compile.
pub fn parse_selectors(selectors: &str) -> Result<Vec<Selector>, SelectorError> {
    let mut sequences: Vec<Selector> = Vec::new();
    // Iterate through selectors, which are separated by commas
    for selector in selectors.split(",") {
        let mut sequence = Selector::new()?;
        // Iterate through components in an individual selector, which are separated by colons
        for (idx, component) in selector.split(":").enumerate() {
            // If component is empty, we do nothing
            if component.is_empty() {
                continue;
            }
            // Try to parse int from component. If we're successful, use that int as a start index,
            // end index, or step. If parse() returns an error, use that component as a regex
            // pattern to match to
            let parsed_component = component.parse::<usize>();
            match parsed_component {
                Ok(_ok) => {
                    let raw_number = parsed_component.as_ref().unwrap();
                    match idx {
                        0 => {
                            // Subtract 1 from start index for 1-based to 0-based conversion
                            sequence.start_idx = raw_number - 1;
                            // If this is the full selection, set this to the end index as well
                            if selector.matches(":").count() == 0 {
                                sequence.end_idx = raw_number - 1;
                            }
                        }
                        1 => {
                            // Subtract 1 from end index for 1-based to 0-based conversion
                            sequence.end_idx = raw_number - 1;
                        }
                        2 => {
                            // Step value should NOT be decremented - use raw value
                            sequence.step = *raw_number;
                        }
                        _ => panic!("A selector cannot be more than three components long"),
                    }
                }
                Err(_e) => {
                    let case_insensitive_regex = format!(r"(?i).*{}.*", &component);
                    match idx {
                        0 => {
                            sequence.start_regex = Regex::new(&case_insensitive_regex)
                                .map_err(|e| SelectorError::InvalidRegex { 
                                    pattern: case_insensitive_regex.clone(), 
                                    source: e 
                                })?;
                            // Set the start index to the usize max to ensure it doesn't interfere
                            sequence.start_idx = usize::MAX;
                            // If this is the full selection, set this to the end regex as well
                            if selector.matches(":").count() == 0 {
                                sequence.end_regex = Regex::new(&case_insensitive_regex)
                                    .map_err(|e| SelectorError::InvalidRegex { 
                                        pattern: case_insensitive_regex, 
                                        source: e 
                                    })?;
                            }
                        }
                        1 => sequence.end_regex = Regex::new(&case_insensitive_regex)
                            .map_err(|e| SelectorError::InvalidRegex { 
                                pattern: case_insensitive_regex, 
                                source: e 
                            })?,
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
    Ok(sequences)
}

#[cfg(test)]
#[path = "selector_tests.rs"]
mod selector_tests;
