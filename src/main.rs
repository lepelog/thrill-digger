use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::time;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::path::Path;
use std::io::{self, Read};

#[macro_use]
extern crate num_derive;

extern crate num_traits;

use num_traits::FromPrimitive;

pub mod experiments;
use crate::experiments::*;

pub mod rng;
use crate::rng::*;

pub mod thrill_digger;
use crate::thrill_digger::*;

pub mod solver;
use crate::solver::*;

fn main() {
    // println!("{}", get_loop_value_belongs_to(2613080473, &get_all_loops_and_members()));
    test_solver();
}

fn test_solver() {
    let mut solver = ThrillDiggerBoardSolver::new();
    solver.set_difficulty(&HoleMinigameDifficulty::Expert);
    // solver.set_hole(8, HoleContent::BlueRupee);
    // solver.set_hole(2, HoleContent::BlueRupee);
    // solver.set_hole(13, HoleContent::BlueRupee);
    // solver.set_hole(14, HoleContent::BlueRupee);
    // solver.set_hole(15, HoleContent::RedRupee);
    // solver.set_hole(7, HoleContent::BlueRupee);
    // solver.set_hole(19, HoleContent::BlueRupee);
    // solver.set_hole(25, HoleContent::BlueRupee);
    // solver.set_hole(12, HoleContent::Rupoor);
    // solver.set_hole(6, HoleContent::BlueRupee);
    // solver.set_hole(0, HoleContent::BlueRupee);
    // solver.set_hole(11, HoleContent::Bomb);
    // solver.set_hole(10, HoleContent::Bomb);
    // solver.set_hole(17, HoleContent::Bomb);
    // let last_matching_seed = solver.calculate_probabilities();
    // println!("{}", solver.probability_string());
    // let old_time = time::SystemTime::now();
    let boards = get_all_possible_boards(&HoleMinigameDifficulty::Expert);
    // let duration = old_time.elapsed().unwrap();
    // println!("took {}ms", duration.as_millis());
    'outloop: loop {
        println!("input '{{hole number}} {{content}}'");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(error) => {
                println!("error: {}", error);
                return;
            },
            Ok(_) => {
                let mut split = input.split(" ");
                let hole_num = match split.next().unwrap().trim().parse::<usize>() {
                    Err(_) => {
                        println!("invalid number!");
                        continue 'outloop;
                    }
                    Ok(num) => num,
                };
                let hole_content = match split.next().unwrap_or("").trim().parse::<u32>() {
                    Err(_) => {
                        println!("invalid number!");
                        continue 'outloop;
                    }
                    Ok(num) => {
                        match HoleContent::from_u32(num) {
                            None => { println!("{} is not a valid content!", num);
                                continue 'outloop;
                            },
                            Some(hc) => hc,
                        }
                    },
                };
                solver.set_hole(hole_num, hole_content);
                solver.calculate_probabilities_with_pregenerated(&boards);
                println!("{}\n", solver.board_state_to_string());
                println!("{}\n", solver.probability_string());
            }
        }
    }
}

fn get_all_possible_boards(difficulty: &HoleMinigameDifficulty) -> Vec<Vec<[HoleContent; 40]>> {
    let mut result = Vec::new();
    for &(seed, period) in ALL_RNG_LOOPS.iter().filter(|(_, period)| *period > 8) {
        let mut current_boards = Vec::with_capacity(period as usize);
        let mut rng = RngContext::from_state(seed);
        for _ in 0..period {
            let hole_minigame = HoleMinigame::generate(&mut rng.clone(), *difficulty);
            current_boards.push(hole_minigame.get_holes().clone());
            rng.next_u32();
        }
        result.push(current_boards);
    }
    return result;
}