extern crate num_traits;
extern crate ss_rng;

use std::time;

use num_traits::cast::FromPrimitive;
use std::io::{self, Read};

use ss_rng::rng::*;
use ss_rng::solver::*;
use ss_rng::thrill_digger::*;

fn main() {
    // println!("{}", get_loop_value_belongs_to(2613080473, &get_all_loops_and_members()));
}

fn get_special_expert_boards() {
    let mut most_valuable = (0, None);
    let mut least_valueable = (isize::MAX, None);
    let mut count_map = std::collections::HashMap::new();
    let mut holes = [HoleContent::Unspecified; 40];
    for &(seed, period) in ALL_RNG_LOOPS.iter().filter(|(_, period)| *period > 8) {
        let mut rng = RngContext::from_state(seed);
        for _ in 0..period {
            ExpertHoleMinigame::generate(&mut rng.clone(), &mut holes);
            let total_value = ExpertHoleMinigame::get_total_value(&holes);
            *count_map.entry(total_value).or_insert(0) += 1;
            if total_value > most_valuable.0 {
                most_valuable.0 = total_value;
                most_valuable.1 = Some(rng.get_state());
            }
            if total_value < least_valueable.0 {
                least_valueable.0 = total_value;
                least_valueable.1 = Some(rng.get_state());
            }
            rng.next_u32();
        }
    }
    println!("best: {}, worst: {}", most_valuable.0, least_valueable.0);
    let mut sorted_results: Vec<_> = count_map.drain().collect();
    sorted_results.sort_by_key(|(val, cnt)| *val);
    for (val, cnt) in sorted_results {
        println!("{}:{}", val, cnt);
    }
}

#[cfg(test)]
mod test {
    use crate::get_special_expert_boards;
    #[test]
    fn name() {
        get_special_expert_boards();
    }
}
