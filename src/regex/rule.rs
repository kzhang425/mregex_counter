
pub enum PatternType {
    Alphabetic,
    Numeric,
    Everything,
    Specific(char),
}

impl PatternType {
    pub fn is_of_type(&self, character: char) -> bool {
        match self {
            Self::Alphabetic => {
                if character.is_alphabetic() {
                    return true;
                } else {
                    return false;
                }
            }

            Self::Numeric => {
                if character.is_numeric() {
                    return true;
                } else {
                    return false;
                }
            }

            Self::Everything => {
                true
            }

            Self::Specific(c) => {
                if character == *c {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

pub struct State {
    min: usize,
    max: Option<usize>,
    patterns: Vec<PatternType>, // The aim of this is to provide some OR functionality.
}

impl State {
    /// Creates a new State object.
    pub fn new(min: usize, max: Option<usize>, states: Vec<PatternType>) -> Self {
        Self {
            min,
            max,
            patterns: states,
        }
    }

    /// Checks if a character works for the state
    pub fn does_char_qualify(&self, character: char) -> bool {
        for pattern in self.patterns.iter() {
            if pattern.is_of_type(character) {
                return true;
            }
        }
        false
    }

    /// Don't really use this function out of context. Doesn't work how you would expect it to.
    pub fn within_count(&self, count: usize) -> bool {
        match self.max {
            None => {
                // Just check if it is equal to or greater than min
                count >= self.min
            }

            Some(max) => {
                count >= self.min && count <= max
            }
        }
    }

    pub fn within_upper_count(&self, count: usize) -> bool {
        match self.max {
            None => {
                true
            }

            Some(max_num) => {
                count <= max_num
            }
        }
    }

    pub fn allows_skip(&self) -> bool {
        if self.min == 0 {
            true
        } else {
            false
        }
    }
}