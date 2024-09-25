use mregex_counter::regex::engine::*;
use mregex_counter::regex::rule::*;

fn main() {
    let block_states = vec![State::new(1, Some(1), vec![PatternType::Alphabetic]), State::new(1, Some(1), vec![PatternType::Numeric])];
    let block_state = State::new_coalesce_block(1, Some(4), block_states);
    let mut engine = Engine::new("123".to_string(), vec![block_state]);
    engine.process();
    println!("{:#?}", engine.extract_results());
}
