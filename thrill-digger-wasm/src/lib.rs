use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate num_traits;
use num_traits::FromPrimitive;

extern crate ss_rng;
use ss_rng::solver::ThrillDiggerBoardSolver;
use ss_rng::thrill_digger::*;
use ss_rng::rng::{ALL_RNG_LOOPS, RngContext};

#[wasm_bindgen]
pub struct SolverWrapper {
    inner: ThrillDiggerBoardSolver,
    cached_boards: Vec<Vec<[HoleContent; 40]>>,
}

impl From<ThrillDiggerBoardSolver> for SolverWrapper {
    fn from(inner: ThrillDiggerBoardSolver) -> SolverWrapper {
        SolverWrapper {
            inner,
            cached_boards: Vec::new(),
        }
    }
}

#[wasm_bindgen]
impl SolverWrapper {
    #[wasm_bindgen]
    pub fn set_hole(&mut self, hole: u32, content: u32) -> Result<(), JsValue>{
        self.inner.set_hole(hole as usize, FromPrimitive::from_u32(content).ok_or(JsValue::from("not a valid content!"))?);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn calculate_probabilities(&mut self) {
        self.inner.calculate_probabilities();
    }

    pub fn calculate_probabilities_with_pregenerated(&mut self) {
        self.inner.calculate_probabilities_with_pregenerated(&self.cached_boards)
    }

    #[wasm_bindgen]
    pub fn get_probability(&self, slot: u32) -> f32 {
        self.inner.get_bomb_probability(slot as usize).unwrap_or(0f32)
    }


    #[wasm_bindgen]
    pub fn cache_boards(&mut self) {
        self.cached_boards.clear();
        for &(seed, period) in ALL_RNG_LOOPS.iter().filter(|(_, period)| *period > 8) {
            let mut current_boards = vec![[HoleContent::Unspecified; 40]; period as usize];
            let mut rng = RngContext::from_state(seed);
            let mut hole_minigame = HoleMinigame::new(self.inner.get_difficulty());
            for board in current_boards.iter_mut() {
                hole_minigame.regenerate(&mut rng.clone());
                for (field_num, field) in hole_minigame.get_holes().iter().enumerate() {
                    board[field_num] = *field;
                }
                rng.next_u32();
            }
            self.cached_boards.push(current_boards);
        }
    }

    #[wasm_bindgen]
    pub fn get_width(&self) -> isize {
        self.inner.get_difficulty().get_board_width()
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> isize {
        self.inner.get_difficulty().get_board_height()
    }

    #[wasm_bindgen]
    pub fn get_hole_count(&self) -> isize {
        self.inner.get_difficulty().get_hole_count()
    }
}

#[wasm_bindgen]
pub fn create_solver(difficulty: u32) -> Result<SolverWrapper, JsValue> {
    let parsed_dif = match difficulty {
        0 => HoleMinigameDifficulty::Beginner,
        1 => HoleMinigameDifficulty::Intermediate,
        2 => HoleMinigameDifficulty::Expert,
        _ => return Err(JsValue::from_str("invalid difficulty!")),
    };
    let mut solver = ThrillDiggerBoardSolver::new();
    solver.set_difficulty(&parsed_dif);
    return Ok(solver.into());
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