extern crate num_traits;
extern crate rand;
extern crate ss_rng;

use std::collections::HashMap;
use std::time;

use num_traits::cast::FromPrimitive;
use std::io::{self, Read};

use rand::prelude::*;

use ss_rng::rng::*;
use ss_rng::solver::*;
use ss_rng::thrill_digger::*;

fn main() {
    // println!("{}", get_loop_value_belongs_to(2613080473, &get_all_loops_and_members()));
    do_test();
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

// [(0, 1824), (1, 1347), (2, 1007), (3, 661), (4, 473), (5, 354), (6, 294), (7, 271), (8, 214), (9, 149), (10, 98), (11, 37), (12, 13), (13, 4), (15, 1), (41, 3253)]
pub fn do_test() {
    let mut solver = ThrillDiggerExpertSolver::new();
    solver.cache_boards();

    let mut guess_map = HashMap::new();
    let mut rng = thread_rng();
    for iteration in 0..1000 {
        let mut current_guess_count = 0;
        let mut current_board = get_random_board(&mut rng, &solver.cached_boards);
        solver.input.clear_board();
        solver.input.clear_selected_loops();
        solver.output.clear_possible_loops();
        for step in 0..40 {
            solver.input.selected_loops = solver.output.possible_loops;
            // dbg!(&solver.input);
            // dbg!(&solver.output);
            solver.calc_update();
            // select
            let mut selected_slot = None;
            let mut min_bomb = 1f32;
            if solver
                .input
                .current_board_state
                .iter()
                .filter(|h| {
                    matches!(
                        **h,
                        HoleContent::Unspecified | HoleContent::Bomb | HoleContent::Rupoor
                    )
                })
                .count()
                == 16
            {
                // board is solved!
                // println!("solved it!! {:?}", solver.input.current_board_state);
                // marker for "solved board"
                current_guess_count = 41;
                break;
            }
            for (slot, hole) in solver.input.current_board_state.iter().enumerate() {
                if *hole == HoleContent::Unspecified {
                    let bomb_prob = solver.output.all_probabilities[HoleContent::Bomb as usize][slot];
                    if bomb_prob < min_bomb {
                        if true {
                            selected_slot = Some(slot);
                            min_bomb = bomb_prob;
                        }
                    }
                }
            }
            if let Some(slot) = selected_slot {
                // dig up the hole
                let mut content = current_board[slot];
                if content == HoleContent::Bomb {
                    if step == 0 {
                        // first dig bomb reshuffles the board
                        current_board = get_random_board(&mut rng, &solver.cached_boards);
                        content = current_board[slot];
                        if content == HoleContent::Bomb {
                            // ...but it can still be a bomb
                            break;
                        }
                    } else {
                        // :(
                        break;
                    }
                }
                solver.input.current_board_state[slot] = content;
                current_guess_count += 1;
            } else {
                println!("solved it? {:?}", solver.input.current_board_state);
                break;
            }
        }
        *guess_map.entry(current_guess_count).or_insert(0) += 1;
        println!("done iter {}", iteration);
    }
    println!("{:?}", guess_map);
    let mut sorted: Vec<_> = guess_map.drain().collect();
    sorted.sort_by_key(|s| s.0);
    println!("{:?}", sorted);
}

pub fn get_random_board<R: Rng>(
    rng: &mut R,
    boards: &Vec<Vec<[HoleContent; 40]>>,
) -> Vec<HoleContent> {
    let board_count: usize = boards.iter().map(|b| b.len()).sum();
    let mut pos = rng.gen_range(0..board_count);
    for board_list in boards.iter() {
        if pos < board_list.len() {
            return board_list[pos].into();
        }
        pos -= board_list.len();
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use crate::get_special_expert_boards;
    #[test]
    fn name() {
        get_special_expert_boards();
    }
}
