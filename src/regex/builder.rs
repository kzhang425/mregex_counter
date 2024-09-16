use super::engine::*;
use super::rule::State;


// Basic string parser that builds the rule states for use with the engine. Hard-coded version of regex that allows custom regex build.

pub struct Builder<'a> {
    input_string: String,
    interpreted_substrings: Vec<(State, &'a str)>,
}

impl Builder<'_> {
    pub fn new(input_string: String) -> Self {
        Self {
            input_string,
            interpreted_substrings: Vec::new(),
        }
    }
}