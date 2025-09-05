use regex::Regex;
use std::fmt;
include!("utils.rs");

#[derive(Debug)]
pub enum SelectorError {
    InvalidRegex {
        pattern: String,
        source: regex::Error,
    },
    InvalidSelector {
        selector: String,
        reason: String,
    },
}

impl fmt::Display for SelectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SelectorError::InvalidRegex { pattern, source } => {
                write!(f, "Invalid regex pattern '{}': {}", pattern, source)
            }
            SelectorError::InvalidSelector { selector, reason } => {
                write!(f, "Invalid selector '{}': {}", selector, reason)
            }
        }
    }
}

impl std::error::Error for SelectorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SelectorError::InvalidRegex { source, .. } => Some(source),
            SelectorError::InvalidSelector { .. } => None,
        }
    }
}

/// Keep track of user column and row selections
#[derive(Debug)]
pub struct Selector {
    /// Index of first row to grab (start of range) - can be negative for Python-style indexing
    pub start_idx: i64,

    /// Resolved start index (converted from negative to positive if needed)
    pub resolved_start_idx: usize,

    /// Regex of first to grab (start of range)
    pub start_regex: regex::Regex,

    /// Index of last row to grab (end of range) - can be negative for Python-style indexing
    pub end_idx: i64,

    /// Resolved end index (converted from negative to positive if needed)
    pub resolved_end_idx: usize,

    /// Regex of last row to grab (end of range)
    pub end_regex: regex::Regex,

    /// Step size between start and end of range
    pub step: usize,

    /// Keep track of when to stop adding rows from range to output
    pub stopped: bool,

    /// Track if indices have been resolved for a given collection length
    pub indices_resolved: bool,
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
        let start_regex = Regex::new(default_regex).map_err(|e| SelectorError::InvalidRegex {
            pattern: default_regex.to_string(),
            source: e,
        })?;
        let end_regex = Regex::new(default_regex).map_err(|e| SelectorError::InvalidRegex {
            pattern: default_regex.to_string(),
            source: e,
        })?;

        Ok(Selector {
            start_idx: 0,
            resolved_start_idx: 0,
            start_regex,
            end_idx: i64::MAX,
            resolved_end_idx: usize::MAX,
            end_regex,
            step: 1,
            stopped: false,
            indices_resolved: false,
        })
    }

    /// Resolve negative indices based on collection length (Python-style indexing)
    ///
    /// # Arguments
    ///
    /// * `collection_length` - The length of the collection being indexed
    ///
    /// # Examples
    ///
    /// * `-1` with length 5 becomes index 4 (last item)
    /// * `-2` with length 5 becomes index 3 (second to last)
    ///
    /// Negative end indices are treated as exclusive bounds, so an end index of
    /// `-1` with length 5 resolves to `3`, excluding the last item.
    pub fn resolve_indices(&mut self, collection_length: usize) {
        if self.indices_resolved {
            return;
        }

        // Resolve start index
        self.resolved_start_idx = if self.start_idx < 0 {
            let abs_idx = (-self.start_idx) as usize;
            if abs_idx > collection_length {
                0 // Out of bounds negative index, clamp to start
            } else {
                collection_length.saturating_sub(abs_idx)
            }
        } else if self.start_idx == i64::MAX {
            usize::MAX // Keep as usize::MAX for regex-based selection
        } else if self.start_idx == 0 {
            0 // Handle the special case of 0 (keep as 0, don't convert)
        } else {
            (self.start_idx - 1) as usize // Convert 1-based to 0-based for positive indices
        };

        // Resolve end index
        self.resolved_end_idx = if self.end_idx < 0 {
            let abs_idx = (-self.end_idx) as usize;
            if abs_idx > collection_length {
                self.resolved_start_idx = usize::MAX;
                usize::MAX // Out of bounds negative index, yield no matches
            } else {
                let idx = collection_length.saturating_sub(abs_idx);
                if self.start_idx != self.end_idx {
                    idx.saturating_sub(1)
                } else {
                    idx
                }
            }
        } else if self.end_idx == i64::MAX {
            usize::MAX // Keep as usize::MAX for regex-based or unlimited selection
        } else if self.end_idx == 0 {
            0 // Handle the special case of 0 (keep as 0, don't convert)
        } else {
            (self.end_idx - 1) as usize // Convert 1-based to 0-based for positive indices
        };

        if self.resolved_start_idx > self.resolved_end_idx {
            self.resolved_start_idx = usize::MAX;
            self.resolved_end_idx = usize::MAX;
        }

        self.indices_resolved = true;
    }

    /// Reset resolution state - call when reusing selector for new collection
    pub fn reset_resolution(&mut self) {
        self.indices_resolved = false;
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
            && self.resolved_start_idx == other.resolved_start_idx
            && utils::regex_eq(&self.start_regex, &other.start_regex)
            && self.end_idx == other.end_idx
            && self.resolved_end_idx == other.resolved_end_idx
            && utils::regex_eq(&self.end_regex, &other.end_regex)
            && self.step == other.step
            && self.stopped == other.stopped
            && self.indices_resolved == other.indices_resolved
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
            let parsed_component = component.parse::<i64>();
            match parsed_component {
                Ok(_ok) => {
                    let raw_number = parsed_component.as_ref().unwrap();
                    match idx {
                        0 => {
                            // Store raw signed number - will be resolved later with collection length
                            sequence.start_idx = *raw_number;
                            // If this is the full selection, set this to the end index as well
                            if selector.matches(":").count() == 0 {
                                sequence.end_idx = *raw_number;
                            }
                        }
                        1 => {
                            // Store raw signed number - will be resolved later with collection length
                            sequence.end_idx = *raw_number;
                        }
                        2 => {
                            // Step value should NOT be negative and must be positive
                            if *raw_number <= 0 {
                                return Err(SelectorError::InvalidSelector {
                                    selector: selector.to_string(),
                                    reason:
                                        "step size must be a positive integer greater than zero."
                                            .to_string(),
                                });
                            }
                            sequence.step = *raw_number as usize;
                        }
                        _ => {
                            return Err(SelectorError::InvalidSelector {
                                selector: selector.to_string(),
                                reason: "A selector cannot be more than three components long"
                                    .to_string(),
                            })
                        }
                    }
                }
                Err(_e) => {
                    let case_insensitive_regex = format!(r"(?i).*{}.*", &component);
                    match idx {
                        0 => {
                            sequence.start_regex =
                                Regex::new(&case_insensitive_regex).map_err(|e| {
                                    SelectorError::InvalidRegex {
                                        pattern: case_insensitive_regex.clone(),
                                        source: e,
                                    }
                                })?;
                            // Set the start index to the i64 max to ensure it doesn't interfere
                            sequence.start_idx = i64::MAX;
                            // If this is the full selection, set this to the end regex as well
                            if selector.matches(":").count() == 0 {
                                sequence.end_regex =
                                    Regex::new(&case_insensitive_regex).map_err(|e| {
                                        SelectorError::InvalidRegex {
                                            pattern: case_insensitive_regex,
                                            source: e,
                                        }
                                    })?;
                            }
                        }
                        1 => {
                            sequence.end_regex =
                                Regex::new(&case_insensitive_regex).map_err(|e| {
                                    SelectorError::InvalidRegex {
                                        pattern: case_insensitive_regex,
                                        source: e,
                                    }
                                })?
                        }
                        2 => {
                            return Err(SelectorError::InvalidSelector {
                                selector: selector.to_string(),
                                reason: "Step size must be an integer".to_string(),
                            })
                        }
                        _ => {
                            return Err(SelectorError::InvalidSelector {
                                selector: selector.to_string(),
                                reason: "A selector cannot be more than three components long"
                                    .to_string(),
                            })
                        }
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
