#[derive(Clone, Copy)]
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

#[derive(Clone)]
pub struct State {
    min: usize,
    max: Option<usize>,
    patterns: Vec<PatternType>, // The aim of this is to provide some OR functionality.
    block: Option<Vec<State>>,
}

impl State {
    /// Creates a new State object. This cannot create a block type state.
    pub fn new(min: usize, max: Option<usize>, patterns: Vec<PatternType>) -> Self {
        Self {
            min,
            max,
            patterns,
            block: None,
        }
    }

    pub fn new_block(min: usize, max: Option<usize>, states: Vec<State>) -> Self {
        Self {
            min,
            max,
            patterns: Vec::new(),
            block: Some(states),
        }
    }

    pub fn is_block_type(&self) -> bool {
        self.block.is_some()
    }

    pub fn get_max(&self) -> Option<usize> {
        self.max
    }

    pub fn get_min(&self) -> usize {
        self.min
    }

    pub fn block_size(&self) -> Option<usize> {
        if let Some(b) = self.get_block_states() {
            Some(b.len())
        } else {
            None
        }
    }

    fn get_block_states(&self) -> Option<&Vec<State>> {
        if let None = self.block {
            None
        } else {
            self.block.as_ref()
        }
    }

    pub fn expand_block_states(&self) -> Option<Vec<State>> {
        Some(self.get_block_states()?.clone())
    }

    /// Checks if a character works for the state
    pub fn does_char_qualify(&self, character: char) -> bool {
        if self.is_block_type() {
            let block_vec_opt = self.get_block_states();
            if let None = block_vec_opt {
                return false;
            }

            // Otherwise, we can start checking. 
            let block_vec = block_vec_opt.unwrap();
            for state in block_vec.iter() {
                let check_result = state.does_char_qualify(character);
                if !state.allows_skip() {
                    return check_result;
                }
                // Else, we would try the next state if it can be skipped.
                if check_result {
                    return true;
                }
            }

            return false;
        }

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