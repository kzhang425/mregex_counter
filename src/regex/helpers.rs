
/// Used by the engine to cache where it is at a certain point in the algorithm.
#[derive(Clone)]
pub struct Cache {
    counts: Vec<usize>,
    cur_pos: usize,
}

impl Cache {
    pub fn new(counts: Vec<usize>, cur_pos: usize) -> Self {
        Self {
            counts,
            cur_pos,
        }
    }

    pub fn extract(self) -> (Vec<usize>, usize) {
        (self.counts, self.cur_pos)
    }
}