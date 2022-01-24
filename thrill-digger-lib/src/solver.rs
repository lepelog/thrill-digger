use std::fmt::Write;

use crate::rng::*;
use crate::thrill_digger::*;

pub struct ThrillDiggerBoardSolver {
    current_difficulty: HoleMinigameDifficulty,
    current_board_state: Vec<HoleContent>,
    bomb_probabilities: Vec<f32>,
    rupoor_probabilities: Vec<f32>,
    possible_loops: [bool; BIG_LOOP_COUNT],
    possible_rng_values_count: usize,
}

impl ThrillDiggerBoardSolver {
    pub fn new() -> Self {
        ThrillDiggerBoardSolver {
            current_difficulty: HoleMinigameDifficulty::Beginner,
            current_board_state: vec![
                HoleContent::Unspecified;
                HoleMinigameDifficulty::Beginner.get_hole_count() as usize
            ],
            bomb_probabilities: Vec::new(),
            rupoor_probabilities: Vec::new(),
            possible_loops: [true; BIG_LOOP_COUNT],
            possible_rng_values_count: 0,
        }
    }

    pub fn clear(&mut self) {
        for slot in self.current_board_state.iter_mut() {
            *slot = HoleContent::Unspecified;
        }
    }

    pub fn reset_possible_loops(&mut self) {
        for i in self.possible_loops.iter_mut() {
            *i = true;
        }
    }

    pub fn get_possible_loops(&self) -> &[bool; BIG_LOOP_COUNT] {
        &self.possible_loops
    }

    pub fn set_possible_loop(&mut self, loop_idx: u32, set_state: bool) {
        self.possible_loops[loop_idx as usize] = set_state;
    }

    pub fn set_difficulty(&mut self, difficulty: &HoleMinigameDifficulty) {
        self.current_difficulty = *difficulty;
        self.current_board_state =
            vec![HoleContent::Unspecified; difficulty.get_hole_count() as usize];
        self.bomb_probabilities = vec![0.0f32; difficulty.get_hole_count() as usize];
        self.rupoor_probabilities = vec![0.0f32; difficulty.get_hole_count() as usize];
    }

    pub fn set_hole(&mut self, slot: usize, content: HoleContent) {
        if slot < self.current_board_state.len() {
            self.current_board_state[slot] = content;
        }
    }

    // not used anymore since it's way slower than just pregenerating all boards at the start
    // pub fn calculate_probabilities(&mut self) -> u32 {
    //     let mut matching_count = 0;
    //     let mut bomb_counts = vec![0; self.current_difficulty.get_hole_count() as usize];
    //     let mut rupoor_counts = vec![0; self.current_difficulty.get_hole_count() as usize];
    //     let mut last_matching_seed = 0;
    //     let mut last_matching_loop_idx = 0;
    //     let mut hole_minigame = HoleMinigame::new(&self.current_difficulty);
    //     for (rng_loop_idx, &(seed, period)) in ALL_RNG_LOOPS.iter().filter(|(_, period)| *period > 8).enumerate() {
    //         let mut rng = RngContext::from_state(seed);
    //         for _ in 0..period {
    //             hole_minigame.regenerate(&mut rng.clone());
    //             if hole_minigame.check_equals(&self.current_board_state) {
    //                 last_matching_seed = seed;
    //                 last_matching_loop_idx = rng_loop_idx;
    //                 matching_count += 1;
    //                 for (i, hole_content) in hole_minigame.get_holes().iter().enumerate() {
    //                     if *hole_content == HoleContent::Bomb {
    //                         bomb_counts[i] += 1;
    //                     }
    //                     if *hole_content == HoleContent::Rupoor {
    //                         rupoor_counts[i] += 1;
    //                     }
    //                 }
    //             }
    //             rng.next_u32();
    //         }
    //     }
    //     if matching_count == 1 {
    //         self.identified_loop = Some(last_matching_loop_idx);
    //     }
    //     if matching_count >= 1 {
    //         for i in 0..self.current_difficulty.get_hole_count() as usize {
    //             self.bomb_probabilities[i] = bomb_counts[i] as f32 / matching_count as f32;
    //             self.rupoor_probabilities[i] = rupoor_counts[i] as f32 / matching_count as f32;
    //         }
    //     } else {
    //         for bomb_prob in self.bomb_probabilities.iter_mut() {
    //             *bomb_prob = f32::NAN;
    //         }
    //         for rupoor_prob in self.rupoor_probabilities.iter_mut() {
    //             *rupoor_prob = f32::NAN;
    //         }
    //     }
    //     return last_matching_seed;
    // }

    pub fn calculate_probabilities_with_pregenerated(
        &mut self,
        boards: &Vec<Vec<[HoleContent; 40]>>,
    ) {
        let mut matching_count = 0;
        let mut bomb_counts = vec![0; self.current_difficulty.get_hole_count() as usize];
        let mut rupoor_counts = vec![0; self.current_difficulty.get_hole_count() as usize];
        let mut found_loops = [false; BIG_LOOP_COUNT];
        for (rng_loop_idx, loop_boards) in boards.iter().enumerate() {
            // if we are locked to a loop index, skip all other loops
            if !self.possible_loops[rng_loop_idx] {
                continue;
            }
            for board in loop_boards.iter() {
                if board
                    .iter()
                    .zip(&self.current_board_state)
                    .all(|(is, should)| *should == HoleContent::Unspecified || is == should)
                {
                    found_loops[rng_loop_idx] = true;
                    matching_count += 1;
                    for (i, hole_content) in board.iter().enumerate() {
                        if *hole_content == HoleContent::Bomb {
                            bomb_counts[i] += 1;
                        } else if *hole_content == HoleContent::Rupoor {
                            rupoor_counts[i] += 1;
                        }
                    }
                }
            }
        }
        self.possible_loops = found_loops;
        self.possible_rng_values_count = matching_count;
        for i in 0..self.current_difficulty.get_hole_count() as usize {
            self.bomb_probabilities[i] = bomb_counts[i] as f32 / matching_count as f32;
            self.rupoor_probabilities[i] = rupoor_counts[i] as f32 / matching_count as f32;
        }
    }

    pub fn probability_string(&self) -> String {
        let mut out = String::new();
        for y in 0..self.current_difficulty.get_board_height() {
            for x in 0..self.current_difficulty.get_board_width() {
                write!(
                    out,
                    "{:01.2}  ",
                    self.bomb_probabilities
                        [(y * self.current_difficulty.get_board_width() + x) as usize]
                )
                .unwrap();
            }
            writeln!(out).unwrap();
        }
        out
    }

    pub fn board_state_to_string(&self) -> String {
        let mut out = String::new();
        for i in 0..self.current_difficulty.get_board_height() {
            for j in 0..self.current_difficulty.get_board_width() {
                write!(
                    out,
                    "   {}  ",
                    self.current_board_state
                        [(i * self.current_difficulty.get_board_width() + j) as usize]
                        .draw()
                )
                .unwrap();
            }
            writeln!(out).unwrap();
        }
        out
    }

    pub fn get_bomb_probability(&self, slot: usize) -> Option<f32> {
        self.bomb_probabilities.get(slot).copied()
    }

    pub fn get_rupoor_probability(&self, slot: usize) -> Option<f32> {
        self.rupoor_probabilities.get(slot).copied()
    }

    pub fn get_possible_rng_values_count(&self) -> usize {
        self.possible_rng_values_count
    }

    pub fn get_difficulty(&self) -> &HoleMinigameDifficulty {
        &self.current_difficulty
    }

    pub fn get_probabilities(&self) -> &Vec<f32> {
        &self.bomb_probabilities
    }
}
