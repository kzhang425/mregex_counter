use super::rule::State;
use super::helpers::Cache;


pub struct Engine {
    states: Vec<State>,
    input_string: String,

    // Dynamic vector in which we keep track of progress in case we backtrack. Each entry corresponds to one in the states field.
    // Counts how many characters were recognized per state.
    counts: Vec<usize>,

    // pointer to where we are at, points to the character at index for input_string.
    cur_char_pos: usize,

}

impl Engine {
    /// Generates a new Engine instance. 
    pub fn new(input_string: String, states: Vec<State>) -> Self {
        let state_count = states.len();
        Self {
            states,
            input_string,
            counts: Vec::with_capacity(state_count),
            cur_char_pos: 0,
        }
    }

    // $$--------- HELPER FUNCTION SECTION START ----------$$
    #[inline]
    pub fn get_input_string(&self) -> &String {
        &self.input_string
    }

    #[inline]
    pub fn input_string_len(&self) -> usize {
        self.get_input_string().len()
    }

    #[inline]
    pub fn get_states(&self) -> &Vec<State> {
        &self.states
    }

    #[inline]
    pub fn get_states_mut(&mut self) -> &mut Vec<State> {
        &mut self.states
    }

    #[inline]
    pub fn get_counts(&self) -> &Vec<usize> {
        &self.counts
    }

    #[inline]
    pub fn get_cur_pos(&self) -> usize {
        self.cur_char_pos
    }

    #[inline]
    pub fn advance(&mut self) -> bool {
        if self.get_cur_pos() >= (self.get_input_string().len() - 1) {
            false // Can't advance
        } else {
            *self.get_cur_pos_mut() += 1;
            true
        }
    }

    #[inline]
    pub fn finish(&mut self) {
        *self.get_cur_pos_mut() = self.get_input_string().len()
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.get_cur_pos() >= self.get_input_string().len()
    }

    #[inline]
    pub fn cursor_is_at_end(&self) -> bool {
        self.get_cur_pos() == (self.get_input_string().len() - 1)
    }

    #[inline]
    pub fn advance_state(&mut self) -> bool {
        if self.get_counts().len() >= self.get_states().len() {
            false
        } else {
            self.get_counts_mut().push(0);
            true
        }
    }

    #[inline]
    pub fn get_cur_char(&self) -> Option<char> {
        self.get_input_string().chars().nth(self.get_cur_pos())
    }

    #[inline]
    pub fn get_counts_mut(&mut self) -> &mut Vec<usize> {
        &mut self.counts
    }

    #[inline]
    pub fn add_current_count(&mut self) -> Result<(), &'static str> {
        let ptr = self.get_counts_mut().iter_mut().last().ok_or("Failed to retrieve current count.")?;
        *ptr += 1;

        Ok(())
    }

    #[inline]
    pub fn get_cur_pos_mut(&mut self) -> &mut usize {
        &mut self.cur_char_pos
    }

    #[inline]
    pub fn get_current_state(&self) -> Option<&State> {
        self.get_states().get(self.get_counts().len() - 1)
    }

    #[inline]
    pub fn get_next_state(&self) -> Option<&State> {
        self.get_states().get(self.get_counts().len())
    }

    fn cache_generate(&self) -> Cache {
        Cache::new(self.get_counts().clone(), self.get_cur_pos())
    }

    fn cache_consume(&mut self, cache: Cache) {
        (*self.get_counts_mut(), *self.get_cur_pos_mut()) = cache.extract();
    }

    fn cache_consume_non_dropping(&mut self, cache: &Cache) {
        self.cache_consume(cache.clone());
    }

    /// Called when handling a block
    fn expand_block_state(&mut self, multiplicity: usize) -> Result<(), &'static str> {
        if let Some(state) = self.get_current_state() {
            // Some sanity checks
            if !state.is_block_type() {
                return Err("The state is not of a block type.");
            }

            if !state.within_count(multiplicity) {
                return Err("Multiplicity is not valid.");
            }

            // Else, let's go ahead and do this
            let current_index = self.get_counts().len() - 1;
            let mut new_states = Vec::from(&self.get_states()[0..current_index]);

            
            let mut remaining_states = if current_index >= (self.get_states().len() - 1) {
                Vec::new()
            } else {
                Vec::from(&self.get_states()[(current_index + 1)..self.get_states().len()])
            };
            let repr_states = state.expand_block_states().unwrap();

            for _i in 0..multiplicity {
                new_states.append(&mut repr_states.clone())
            }

            // Now cap the end again.
            new_states.append(&mut remaining_states);

            *self.get_states_mut() = new_states;


            Ok(())

        } else {
            Err("Passed in a null state to expand_block_state.")
        }
    }

    //          $$---------- HELPER FUNCTION SECTION END ----------$$
    // --------------------------------------------------------------------------------------------------------------------------------------
    //          $$----------CORE ALGORITHM SECTION START ----------$$

    /// Initializes the struct for a processing run.
    pub fn init(&mut self) -> Result<(), &'static str> {
        let _ = self.get_input_string().chars().nth(0).ok_or("First character not found. Exiting.")?; // Handle a null string, can't regex on it.
        let _ = self.get_states().iter().nth(0).ok_or("No valid states are found to fulfill. Exiting.")?;

        // Make sure we are set up for success, get our first token
        self.get_counts_mut().clear();
        self.get_counts_mut().push(0);
        *self.get_cur_pos_mut() = 0;

        Ok(())
    }

    /// Main algorithmic driver for the Engine instance.
    pub fn process(&mut self) -> Result<bool, &'static str> {
        self.init()?; // If fail initialization, don't bother continuing.
        self.execute()
        
    }

    fn execute(&mut self) -> Result<bool, &'static str> {
        let mut can_qualify_next_state: bool;
        let mut can_qualify_now: bool;
        let mut can_skip_next: bool;
        let mut is_last_state: bool;
        while !self.is_finished() {
            // Check if it qualifies for now
            can_qualify_now = self.evaluate_char_with_limits()?;
            can_qualify_next_state = self.check_if_qualify_for_next_state();
            can_skip_next = match self.get_next_state() {
                None => {
                    false
                }

                Some(st) => {
                    st.allows_skip()
                }
            };
            is_last_state = self.get_next_state().is_none();

            // Block cases are unique, and should be handled foremost. If we hit here, its most likely we handle true/false here.
            if can_qualify_now && self.get_current_state().unwrap().is_block_type() {
                let cache = self.cache_generate();
                let block_state = self.get_current_state().unwrap();
                let state_upper_lim = block_state.get_max().unwrap_or(crate::BLOCK_TRUE_UPPER_LIM);
                let calc_loop_lim = self.input_string_len() / block_state.block_size().unwrap();

                for i in block_state.get_min()..calc_loop_lim.min(state_upper_lim) {
                    self.cache_consume_non_dropping(&cache); // reset
                    self.expand_block_state(i)?;

                    let result = self.execute()?;
                    if result {
                        return Ok(true);
                    }
                }

                // If none of the stuff work above, we simply can't pass
                return Ok(false);

            }

            // Easy case, if you can't qualify now but can qualify on the next state, do that. We account for possibility that 
            // the next state doesn't exist in can_qualify_next_state
            if !can_qualify_now && can_qualify_next_state {
                self.advance_state();
                self.add_current_count()?;
                continue;
            }

            if !can_qualify_now && !can_qualify_next_state {
                // Break down into two cases, one where you can skip the next state and one you can't.

                // If you can't:
                if !can_skip_next {
                    return Ok(false); // out of options.
                } else {
                    // If you can, try that
                    self.advance_state();
                    if !self.advance_state() {
                        // Handles case where we try to push past the last state.
                        return Ok(false);
                    }

                }
            }


            // Easiest case would be if the character can qualify now, but not later.
            if can_qualify_now && !can_qualify_next_state {
                self.add_current_count()?;
                if is_last_state && self.cursor_is_at_end() {
                    self.finish();
                } else {
                    self.advance();
                }
                continue;
            }

            // More complex case
            if can_qualify_now && can_qualify_next_state {
                // We can employ some backtracking to cover our bases
                let cache = self.cache_generate();

                // Let's try to move it to the next state and try
                self.advance_state();
                let result = self.execute()?;
                if result {
                    return Ok(true);
                } else {
                    // Otherwise, we'll just take a safe route and consume it now.
                    self.cache_consume(cache);
                    self.add_current_count()?;
                    self.advance();
                }
            }
        }

        Ok(self.validate())
    }

    fn validate(&self) -> bool {
        if self.get_counts().len() != self.get_states().len() {
            return false;
        }

        for i in (0..self.get_counts().len()) {
            if !self.get_states()[i].within_count(self.get_counts()[i]) {
                return false;
            }
        }

        true
    }

    fn check_if_qualify_for_next_state(&self) -> bool {
        let next = self.get_states().get(self.get_counts().len());
        if let None = next {
            return false; // We have no more
        } else {
            let next_state = next.unwrap();
            let ch = self.get_cur_char();
            if let None = ch {
                return false;
            }
            
            // Otherwise, we see if this is valid.
            let the_ch = ch.unwrap();
            let decision = next_state.does_char_qualify(the_ch) && next_state.within_count(1);
            return decision;
        }
    }

    /// Checks if the current pointer character can qualify for the bound in the state.
    fn evaluate_char_with_limits(&self) -> Result<bool, &'static str> {
        let cur_char = self.get_input_string().chars().nth(self.get_cur_pos()).ok_or("Failed to get current character or out of bounds")?;

        // We also assume that the last element of the counts field corresponds to the current state we're evaluating this character for.
        let state = self.get_current_state().ok_or("Failed to retrieve state based on curent counts information.")?;

        if !state.does_char_qualify(cur_char) {
            // Basically just not what the state expects, quit early here.
            Ok(false)
        } else {
            // Do some more checking. We make sure to increment by one to see if this character can be added to the count
            let last_count = *self.get_counts().last().ok_or("No count was found.")?;
            if state.within_upper_count(last_count + 1) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::rule::PatternType;

    #[test]
    fn simple_tests() {
        let subset = vec![PatternType::Alphabetic];
        let alphabet_rule = vec![State::new(1, Some(4), subset)];
        let mut engine = Engine::new("abcde".to_string(), alphabet_rule);
        let result = engine.process().unwrap();
        assert_eq!(result, false);
    }
}