use std::hash::Hash;
use std::num::NonZeroUsize;

use crate::rng::*;
use crate::thrill_digger::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ExpertSolverInput {
    pub current_board_state: [HoleContent; 40],
    pub selected_loops: [bool; BIG_LOOP_COUNT],
}

impl Default for ExpertSolverInput {
    fn default() -> Self {
        ExpertSolverInput {
            current_board_state: [HoleContent::Unspecified; 40],
            selected_loops: [true; BIG_LOOP_COUNT],
        }
    }
}

impl ExpertSolverInput {
    pub fn clear_board(&mut self) {
        for hole in self.current_board_state.iter_mut() {
            *hole = HoleContent::Unspecified;
        }
    }

    pub fn set_hole(&mut self, slot: usize, content: HoleContent) {
        if let Some(hole) = self.current_board_state.get_mut(slot) {
            *hole = content;
        }
    }

    pub fn get_hole(&self, slot: usize) -> HoleContent {
        self.current_board_state
            .get(slot)
            .copied()
            .unwrap_or(HoleContent::Unspecified)
    }

    pub fn clear_selected_loops(&mut self) {
        for lop in self.selected_loops.iter_mut() {
            *lop = true;
        }
    }

    pub fn set_loop(&mut self, idx: usize, value: bool) {
        if let Some(lop) = self.selected_loops.get_mut(idx) {
            *lop = value;
        }
    }

    pub fn get_loop(&self, idx: usize) -> bool {
        self.selected_loops.get(idx).copied().unwrap_or(false)
    }
}

#[derive(Clone, Debug)]
pub struct ExpertSolverOutput {
    pub all_probabilities: [[f32; 8]; 40],
    pub possible_loops: [bool; BIG_LOOP_COUNT],
    pub possible_rng_values_count: usize,
    pub ranks: [u8; 40],
}

impl Default for ExpertSolverOutput {
    fn default() -> Self {
        ExpertSolverOutput {
            all_probabilities: [[0f32; 8]; 40],
            possible_loops: [false; BIG_LOOP_COUNT],
            possible_rng_values_count: 0,
            ranks: [0; 40],
        }
    }
}

impl ExpertSolverOutput {
    pub fn clear_possible_loops(&mut self) {
        for lop in self.possible_loops.iter_mut() {
            *lop = false;
        }
    }
}

pub fn calculate_probabilities_with_pregenerated(
    boards: &Vec<Vec<[HoleContent; 40]>>,
    input: &ExpertSolverInput,
    output: &mut ExpertSolverOutput,
) {
    let mut matching_count = 0usize;
    let mut all_counts = [[0usize; 8]; 40];
    output.clear_possible_loops();
    for (rng_loop_idx, loop_boards) in boards.iter().enumerate() {
        // if we are locked to a loop index, skip all other loops
        if !input.selected_loops[rng_loop_idx] {
            continue;
        }
        for board in loop_boards.iter() {
            if board
                .iter()
                .zip(&input.current_board_state)
                .all(|(is, should)| *should == HoleContent::Unspecified || is == should)
            {
                output.possible_loops[rng_loop_idx] = true;
                // we know we can't overflow over u32
                matching_count = matching_count.wrapping_add(1);
                for (counts, hole_content) in all_counts.iter_mut().zip(board.iter()) {
                    counts[*hole_content as usize] = counts[*hole_content as usize].wrapping_add(1);
                }
            }
        }
    }
    output.possible_rng_values_count = matching_count;
    for i in 0..40 as usize {
        for j in 1..8 as usize {
            output.all_probabilities[i][j] = all_counts[i][j] as f32 / matching_count as f32;
        }
    }
    // ranking
    // we set all already checked roles to rank 100
    for (rank, content) in output
        .ranks
        .iter_mut()
        .zip(input.current_board_state.iter())
    {
        if *content == HoleContent::Unspecified {
            *rank = 0;
        } else {
            *rank = 100;
        }
    }
    // then we can ignore everything with rank 100 here
    let mut prob_and_ranks: Vec<_> = output
        .all_probabilities
        .iter()
        .zip(output.ranks.iter_mut())
        .filter(|(_, rank)| **rank != 100)
        .collect();
    // sort, so that the lowest probabilities are first
    prob_and_ranks.sort_unstable_by(|(a, _), (b, _)| {
        a[HoleContent::Bomb as usize]
            .total_cmp(&b[HoleContent::Bomb as usize])
            .then_with(|| {
                a[HoleContent::Rupoor as usize].total_cmp(&b[HoleContent::Rupoor as usize])
            })
    });
    // now we can write the ranks in the rank array
    for (index, (_, rank)) in prob_and_ranks.iter_mut().enumerate() {
        **rank = index as u8;
    }
}

pub fn calculate_probabilities_with_pregenerated_cached(
    boards: &Vec<Vec<[HoleContent; 40]>>,
    cache: &mut lru::LruCache<ExpertSolverInput, ExpertSolverOutput>,
    input: &ExpertSolverInput,
    output: &mut ExpertSolverOutput,
) {
    match cache.get(input) {
        Some(out) => output.clone_from(out),
        None => {
            calculate_probabilities_with_pregenerated(boards, input, output);
            // if the number of possible seeds is low enough, the benefit from
            // cache doesn't matter that much, prefer keeping states with
            // a lot of possible seeds in the cache
            if output.possible_rng_values_count > 10000 {
                cache.put(input.clone(), output.clone());
            }
        }
    }
}

pub struct ThrillDiggerExpertSolver {
    pub cached_boards: Vec<Vec<[HoleContent; 40]>>,
    pub cache: lru::LruCache<ExpertSolverInput, ExpertSolverOutput>,
    pub input: ExpertSolverInput,
    pub output: ExpertSolverOutput,
}

impl ThrillDiggerExpertSolver {
    pub fn new() -> Self {
        ThrillDiggerExpertSolver {
            cached_boards: Vec::with_capacity(BIG_LOOP_COUNT),
            cache: lru::LruCache::new(NonZeroUsize::new(100).unwrap()),
            input: ExpertSolverInput::default(),
            output: ExpertSolverOutput::default(),
        }
    }

    pub fn cache_boards(&mut self) {
        self.cached_boards.clear();
        for &(seed, period) in ALL_RNG_LOOPS.iter().filter(|(_, period)| *period > 8) {
            let mut current_boards = vec![[HoleContent::Unspecified; 40]; period as usize];
            let mut rng = RngContext::from_state(seed);
            for board in current_boards.iter_mut() {
                ExpertHoleMinigame::generate(&mut rng.clone(), board);
                rng.next_u32();
            }
            self.cached_boards.push(current_boards);
        }
    }

    pub fn calc_update(&mut self) {
        calculate_probabilities_with_pregenerated_cached(
            &self.cached_boards,
            &mut self.cache,
            &self.input,
            &mut self.output,
        );
        for (selected, possible) in self
            .input
            .selected_loops
            .iter_mut()
            .zip(self.output.possible_loops.iter())
        {
            if !*possible {
                *selected = false;
            }
        }
    }
}
