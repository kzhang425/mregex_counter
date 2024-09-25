use crate::regex::rule::State;
/// Used by the engine to cache where it is at a certain point in the algorithm.
#[derive(Clone)]
pub struct Cache {
    counts: Vec<usize>,
    cur_pos: usize,
    states: Vec<State>,
}

impl Cache {
    pub fn new(counts: Vec<usize>, cur_pos: usize, states: Vec<State>) -> Self {
        Self {
            counts,
            cur_pos,
            states,
        }
    }

    pub fn extract(self) -> (Vec<usize>, usize, Vec<State>) {
        (self.counts, self.cur_pos, self.states)
    }
}