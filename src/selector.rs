use regex::Regex;
include!("utils.rs");

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

impl Default for Selector {
    /// Defaults to implement a new selector without defining each field individually
    fn default() -> Selector {
        Selector {
            // Default start to 0, the first row/column
            start_idx: 0,

            // Default start to ".^", an impossible Regex that nothing will match
            start_regex: Regex::new(r".^").unwrap(),

            // Default end to the max usize value (i.e. 2^64 - 1 on an amd64 machine)
            end_idx: usize::MAX,

            // Default end to ".^", an impossible Regex that nothing will match
            end_regex: Regex::new(r".^").unwrap(),

            // Default step to 1 to get each row
            step: 1,

            // Default stopped to false so we output rows unless otherwise specified
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
            && utils::regex_eq(&self.start_regex, &other.start_regex)
            && self.end_idx == other.end_idx
            && utils::regex_eq(&self.end_regex, &other.end_regex)
            && self.step == other.step
            && self.stopped == other.stopped
    }
}

/// Parse either row or column selectors, turning Python-like list slicing syntax into vector of
/// Selector structs
pub fn parse_selectors(selectors: &str) -> Result<Vec<Selector>, String> {
    let mut sequences: Vec<Selector> = Vec::new();
    // Iterate through selectors, which are separated by commas
    for selector in selectors.split(",") {
        let mut sequence = Selector::default();
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
                            // Prevent division by zero by rejecting step=0
                            if *raw_number == 0 {
                                return Err(format!(
                                    "Invalid selector '{}': step size cannot be zero. Use step=1 to select every item in the range.",
                                    selector
                                ));
                            }
                            sequence.step = *raw_number;
                        }
                        _ => return Err(format!(
                            "Invalid selector '{}': A selector cannot be more than three components long",
                            selector
                        )),
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
                        2 => return Err(format!(
                            "Invalid selector '{}': Step size must be an integer",
                            selector
                        )),
                        _ => return Err(format!(
                            "Invalid selector '{}': A selector cannot be more than three components long",
                            selector
                        )),
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
