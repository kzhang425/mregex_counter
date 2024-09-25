use crate::DEFAULT_ENGINE_TYPICAL_MAX;

use super::engine::*;
use super::rule::{PatternType, State};


// Basic string parser that builds the rule states for use with the engine. Hard-coded version of regex that allows custom regex build.

pub struct Builder {
    input_string: String,
    engine: Engine, // Note that this is not the actual engine that gets outputted. The builder is supposed to output an engine.
    interpreted_substrings: Vec<(State, String)>,
}

impl Builder {
    /// Main tag that should be used to construct the Builder struct
    pub fn init_default(input_string: String) -> Self {
        Self::new(input_string.clone(), Self::default_engine_rebuild(input_string))
    }

    /// Processes the builder provided that there is a valid engine (should always at least have the default) and 
    pub fn process(&mut self) -> Result<bool, &'static str> {
        if self.input_string.len() == 0 {
            return Err("Input string is null, fatal error.");
        }
        if !self.engine.process()? {
            Ok(false)
        } else {
            // Success, and we can extract
            if let Some(results) = self.engine.extract_results() {
                self.interpreted_substrings = results;
                Ok(true)
            } else {
                Err("Unexpected failure, engine ran properly but failed to extract results.")
            }
        }
    }

    /// I used the engine to create the engine.
    fn new(input_string: String, engine: Engine) -> Self {
        Self {
            input_string,
            engine,
            interpreted_substrings: Vec::new(),
        }
    }

    fn default_engine_rebuild(input_string: String) -> Engine {
        let mut main_block = Vec::new(); // Contains everything. Should just be containing iterations of unit block
        let mut unit_block = Vec::new(); // fixed #.# in the beginning, use a coalesce for regular alphabets, and quotes. We will prioritize regular alphabet, then quote.
        let mut coalesce_block = Vec::new(); // Belongs in the unit block
        let mut quote_block = Vec::new();
        unit_block.push(State::new(0, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![PatternType::Numeric]).set_identifier(1));
        unit_block.push(Self::specific_state(1, Some(1), '.').set_identifier(2));
        unit_block.push(State::new(0, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![PatternType::Numeric]).set_identifier(3));


        // Start building the foundational blocks
        // alphabetical is easy, and we can directly put it into the coalesce block, no need for a block
        coalesce_block.push(State::new(1, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![PatternType::Alphabetic]).set_identifier(4));
        
        // Time to build the quote block
        quote_block.push(Self::specific_state(1, Some(1), '"').set_identifier(5));
        quote_block.push(State::new(1, None, vec![PatternType::Everything]).set_identifier(6));
        quote_block.push(Self::specific_state(1, Some(1), '"'));
        let quote_state = State::new_block(1, Some(1), quote_block);
        coalesce_block.push(quote_state);

        // Now for the most complex one
        coalesce_block.push(Self::specific_state(0, Some(1), '(').set_identifier(7)); // Start of grouper

        // Push this coalesce block into the unit block
        unit_block.push(State::new_coalesce_block(1, Some(1), coalesce_block));
        let unit_state = State::new_block(1, Some(1), unit_block);

        // Now we need to coalesce with the right parenthesis operator
        main_block.push(unit_state);
        main_block.push(Self::specific_state(1, Some(1), ')').set_identifier(8));

        let engine_states = vec![State::new_coalesce_block(1, None, main_block)];

        Engine::new(input_string, engine_states)

    }

    fn default_engine(input_string: String) -> Engine {
        let mut core_block = Vec::new(); // This needs to make it into a final state that repeats
        core_block.push(State::new(0, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![PatternType::Numeric]).set_identifier(1)); // First a number, is optional
        core_block.push(State::new(1, Some(1), vec![PatternType::Specific('.')]).set_identifier(2)); // Must have a period
        core_block.push(State::new(0, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![PatternType::Numeric]).set_identifier(3)); // Another number that is optional, representing a range.

        // Now is the hard part, defining what can be put as the regex string.
        let mut inner_block = Vec::new();
        inner_block.push(State::new(1, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![PatternType::Alphabetic]).set_identifier(3)); // Alphabetical stuff. We only have so many

        // Next branch, the quote block
        let mut quote_block = Vec::new();
        let quote_state = State::new(1, Some(1), vec![PatternType::Specific('"')]).set_identifier(4);
        quote_block.push(quote_state.clone()); // Left quote
        quote_block.push(State::new(1, None, vec![PatternType::Everything]).set_identifier(5)); // String of text that will be broken into individual specifics
        quote_block.push(quote_state.set_identifier(6)); // Right quote, type 6

        let quote_state = State::new_block(1, Some(1), quote_block); // It will recognize things like "hello world"

        // Now we collapse everything together for now into the core_block
        inner_block.push(quote_state); // branching for either string of alphabets or a quote block
        let inner_coalesce_state = State::new_coalesce_block(1, Some(1), inner_block);
        core_block.push(inner_coalesce_state);

        // Now, make the entire core_block a state we can define repetitions on. Now that we did #.#<either quote or alphabet chars>, we put this into a potentially repeating block.
        // There is another thing to handle, and that is the case where we use parentheses to repeat multiple core_states.
        let core_state = State::new_block(1, Some(1), core_block); // Will recognize things like 1.2AN or 1.3"Hello world"


        // Building onwards, let's make a block_block, where it will recognize #.#(<anything and any interation of the core_state>)
        let mut block_block = Vec::new();
        block_block.push(State::new(0, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![PatternType::Numeric]).set_identifier(7));
        block_block.push(Self::specific_state(1, Some(1), '.'));
        block_block.push(State::new(0, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![PatternType::Numeric]).set_identifier(8));
        block_block.push(Self::specific_state(1, Some(1), '(').set_identifier(9));
        block_block.push(State::new_block(1, Some(DEFAULT_ENGINE_TYPICAL_MAX), vec![core_state.clone()])); // Ensure we have something in the parentheses to even do stuff with
        block_block.push(Self::specific_state(1, Some(1), ')').set_identifier(10));
        let block_state = State::new_block(1, Some(1), block_block);

        // Coalesce the core and block states
        let combined_state = State::new_coalesce_block(1, None, vec![core_state, block_state]);
        Engine::new(input_string, vec![combined_state])


    }

    /// Helper to reduce coding stuff over and over again.
    #[inline]
    fn specific_state(min: usize, max: Option<usize>, character: char) -> State {
        State::new(min, max, vec![PatternType::Specific(character)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let mut builder = Builder::init_default("1.2A".to_string());
        let test_result = builder.process();
        assert_eq!(test_result.is_ok(), true);

        let mut builder2 = Builder::init_default("1.2\"hello world\"".to_string());
        let test_result2 = builder2.process();
        assert_eq!(test_result2.is_ok(), true);

        let mut builder3 = Builder::init_default("1.2(1.3AB)".to_string());
        let test_result3 = builder3.process();
        assert_eq!(test_result3.is_ok(), true);
    }
}