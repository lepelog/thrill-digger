use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate num_traits;
use num_traits::FromPrimitive;

extern crate ss_rng;
use ss_rng::rng::{RngContext, BIG_LOOP_COUNT};
use ss_rng::solver::ThrillDiggerExpertSolver;

#[wasm_bindgen]
pub struct SolverWrapper {
    inner: ThrillDiggerExpertSolver,
}

impl From<ThrillDiggerExpertSolver> for SolverWrapper {
    fn from(inner: ThrillDiggerExpertSolver) -> SolverWrapper {
        SolverWrapper { inner }
    }
}

#[wasm_bindgen]
impl SolverWrapper {
    #[wasm_bindgen]
    pub fn set_hole(&mut self, hole: u32, content: u32) -> Result<(), JsValue> {
        self.inner.input.set_hole(
            hole as usize,
            FromPrimitive::from_u32(content).ok_or(JsValue::from("not a valid content!"))?,
        );
        Ok(())
    }

    // #[wasm_bindgen]
    // pub fn calculate_probabilities(&mut self) {
    //     self.inner.calculate_probabilities();
    // }

    #[wasm_bindgen]
    pub fn calculate_probabilities_with_pregenerated(&mut self) {
        self.inner.calc_update();
    }

    #[wasm_bindgen]
    pub fn get_probability(&self, slot: u32) -> f32 {
        self.inner
            .output
            .bomb_probabilities
            .get(slot as usize)
            .copied()
            .unwrap_or(0f32)
    }

    #[wasm_bindgen]
    pub fn get_rupoor_probability(&self, slot: u32) -> f32 {
        self.inner
            .output
            .rupoor_probabilities
            .get(slot as usize)
            .copied()
            .unwrap_or(0f32)
    }

    #[wasm_bindgen]
    pub fn get_possible_rng_values_count(&self) -> usize {
        self.inner.output.possible_rng_values_count
    }

    #[wasm_bindgen]
    pub fn reset_possible_loops(&mut self) {
        self.inner.input.clear_selected_loops();
    }

    #[wasm_bindgen]
    pub fn set_possible_loop(&mut self, loop_idx: u32, set_possible: bool) {
        self.inner.input.set_loop(loop_idx as usize, set_possible);
    }

    #[wasm_bindgen]
    pub fn get_possible_loops(&self, possible_loops_out: &mut [u8]) -> Result<(), JsValue> {
        if possible_loops_out.len() != BIG_LOOP_COUNT {
            return Err(JsValue::from_str("array with invalid sized passed in!"));
        }
        for i in 0..BIG_LOOP_COUNT {
            possible_loops_out[i] = self.inner.output.possible_loops[i].into();
        }
        return Ok(());
    }

    #[wasm_bindgen]
    pub fn get_total_loop_count(&self) -> usize {
        self.inner.cached_boards.len()
    }

    #[wasm_bindgen]
    pub fn cache_boards(&mut self) {
        self.inner.cache_boards();
    }

    // for speed/memory reasons only expert currently
    // maybe refactor later
    #[wasm_bindgen]
    pub fn get_width(&self) -> isize {
        8
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> isize {
        5
    }

    #[wasm_bindgen]
    pub fn get_hole_count(&self) -> isize {
        40
    }
}

#[wasm_bindgen]
pub fn create_solver() -> Result<SolverWrapper, JsValue> {
    return Ok(ThrillDiggerExpertSolver::new().into());
}

// #[wasm_bindgen]
// pub fn calculate_boards() {
//     get_all_possible_boards(&HoleMinigameDifficulty::Beginner);
// }

#[wasm_bindgen]
pub fn do_rng_calls(count: u32) -> u32 {
    let mut rng = RngContext::new();
    for _ in 0..count {
        rng.next_u32();
    }
    rng.get_state()
}

// fn get_all_possible_boards(difficulty: &HoleMinigameDifficulty) -> Vec<Vec<[HoleContent; 40]>> {
//     let mut result = Vec::new();
//     for &(seed, period) in ALL_RNG_LOOPS.iter().filter(|(_, period)| *period > 8) {
//         let mut current_boards = Vec::with_capacity(period as usize);
//         let mut rng = RngContext::from_state(seed);
//         for _ in 0..period {
//             let hole_minigame = HoleMinigame::generate(&mut rng.clone(), *difficulty);
//             current_boards.push(hole_minigame.get_holes().clone());
//             rng.next_u32();
//         }
//         result.push(current_boards);
//     }
//     return result;
// }
