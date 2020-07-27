// Seed: 3249224983 (this is just one in the chain)
// cycle length: 1708724
fn main() {
    let mut rng = RngContext {state: 0x945B4001};
    HoleMinigame::generate(&mut rng,HoleMinigameDifficulty::Intermediate).draw();
    // let mut last_state = 3249224983;
    // // let mut holePossibilities = Vec::new();
    // for _i in 0..10 {
    //     rng.set_state(last_state);
    //     last_state = rng.next_u32();
    //     let test = HoleMinigame::generate(&mut rng);
    //     test.draw();
    //     println!();
    //     // holePossibilities.push(test);
    // }
    // let mut bomb_totals = [0; 20];
    // for game in holePossibilities.iter() {
    //     for i in 0..20 {
    //         bomb_totals[i] += if game.holes[i] == HoleContent::Bomb { 1 } else { 0 };
    //     }
    // }
    // for (i, percent) in bomb_totals.iter().map(|v| *v as f32/1708724 as f32).enumerate() {
    //     println!("{}: {}", i, percent);
    // }
}

struct RngContext {
    state: u32,
}

impl RngContext {
    fn next_u32(&mut self) -> u32 {
        let temp1: u64 = self.state as u64 * 1664525u64 + 1013904223u64;
        self.state = (((temp1 >> 0x20) + temp1) & 0xFFFFFFFF) as u32;
        self.state
    }

    fn next_float(&mut self) -> f32 {
        // copied from skyward sword
        f32::from_be_bytes((self.next_u32() >> 9 | 0x3f800000).to_be_bytes()) - 1f32
    }

    fn rng_call_distance(&mut self, start: u32, end: u32) -> u32 {
        self.state = start;
        let mut distance: u32 = 0;
        while self.next_u32() != end {
            distance+=1;
        }
        distance
    }

    fn set_state(&mut self, new_state: u32) {
        self.state = new_state;
    }
}

#[derive(PartialEq, Clone, Copy)]
enum HoleContent {
    Unspecified = 0,
    GreenRupee,
    BlueRupee,
    RedRupee,
    SilverRupee,
    GoldRupee,
    Rupoor,
    Bomb,
}

impl HoleContent {
    fn draw(self) {
        match self {
            HoleContent::Unspecified => print!("?"),
            HoleContent::GreenRupee => print!("G"),
            HoleContent::BlueRupee => print!("B"),
            HoleContent::RedRupee => print!("R"),
            HoleContent::SilverRupee => print!("S"),
            HoleContent::GoldRupee => print!("3"),
            HoleContent::Rupoor => print!("P"),
            HoleContent::Bomb => print!("X"),
        }
    }
}

struct HoleMinigame {
    hole_count: isize,
    board_height: isize,
    board_width: isize,
    rupoor_count: isize,
    bomb_count: isize,
    holes: [HoleContent; 40], // 40 is the max it can be, but I saw a 0x50 (=80) during init, maybe it's bigger? Doesn't matter
    recursion_depth: u8,
}

enum HoleMinigameDifficulty {
    Beginner,
    Intermediate,
    Expert,
}

impl HoleMinigame {
    fn generate(rng: &mut RngContext, difficulty: HoleMinigameDifficulty) -> Self {
        let mut this = match difficulty {
            HoleMinigameDifficulty::Beginner =>  HoleMinigame{
                    hole_count: 20,
                    board_height: 4,
                    board_width: 5,
                    rupoor_count: 0,
                    bomb_count: 4,
                    holes: [HoleContent::Unspecified; 40],
                    recursion_depth: 0,
                },
            HoleMinigameDifficulty::Intermediate => HoleMinigame{
                    hole_count: 30,
                    board_height: 5,
                    board_width: 6,
                    rupoor_count: 4,
                    bomb_count: 4,
                    holes: [HoleContent::Unspecified; 40],
                    recursion_depth: 0,
                },
            HoleMinigameDifficulty::Expert => HoleMinigame{
                    hole_count: 40,
                    board_height: 5,
                    board_width: 8,
                    rupoor_count: 8,
                    bomb_count: 8,
                    holes: [HoleContent::Unspecified; 40],
                    recursion_depth: 0,
                }
        };
        this.try_place(rng);
        return this;
    }

    fn try_place(&mut self, rng: &mut RngContext) {
        for hole in self.holes.iter_mut() {
            *hole = HoleContent::Unspecified;
        }
        self.place_bombs(rng);
        self.place_rupoors(rng);
        let mut bad_pattern = false;
        // check if there is a 2x2 group of "bad stuff" aka bombs and rupoors
        for i in 0..(self.board_height - 1) {
            for j in 0..(self.board_width - 1) {
                if self.is_bomb_or_rupoor(i * self.board_width + j) &&
                self.is_bomb_or_rupoor(i * self.board_width + j + 1) &&
                self.is_bomb_or_rupoor((i + 1) * self.board_width + j) &&
                self.is_bomb_or_rupoor((i + 1) * self.board_width + j + 1) {
                    bad_pattern = true;
                }
            }
        }
        if bad_pattern && self.recursion_depth < 10 {
            self.recursion_depth += 1;
            self.try_place(rng);
            self.recursion_depth -= 1;
        } else {
            self.place_rupees();
        }
    }

    fn place_bombs(&mut self, rng: &mut RngContext) {
        let mut placed_bombs = 0;
        while placed_bombs < self.bomb_count {
            // pick a random field and then check if it's free
            let next_field = (rng.next_float() * self.hole_count as f32) as usize;
            if self.holes[next_field] == HoleContent::Unspecified {
                self.holes[next_field] = HoleContent::Bomb;
                placed_bombs+=1;
            }
        }
    }

    fn place_rupoors(&mut self, rng: &mut RngContext) {
        let mut placed_rupoors = 0;
        while placed_rupoors < self.rupoor_count {
            let next_field = (rng.next_float() * self.hole_count as f32) as usize;
            if self.holes[next_field] == HoleContent::Unspecified {
                self.holes[next_field] = HoleContent::Rupoor;
                placed_rupoors+=1;
            }
        }
    }

    fn place_rupees(&mut self) {
        for i in 0..self.hole_count as isize {
            if !self.is_bomb_or_rupoor(i) {
                // count neighbors...
                let mut neighbors = 0;
                // check if left side
                if i % self.board_width != 0 {
                    if self.is_bomb_or_rupoor(i - self.board_width - 1) {
                        neighbors += 1;
                    }
                    if self.is_bomb_or_rupoor(i - 1) {
                        neighbors += 1;
                    }
                    if self.is_bomb_or_rupoor(i + self.board_width - 1) {
                        neighbors += 1;
                    }
                }
                // check if right side
                if i % self.board_width != (self.board_width - 1) {
                    if self.is_bomb_or_rupoor(i - self.board_width + 1) {
                        neighbors += 1;
                    }
                    if self.is_bomb_or_rupoor(i + 1) {
                        neighbors += 1;
                    }
                    if self.is_bomb_or_rupoor(i + self.board_width + 1) {
                        neighbors += 1;
                    }
                }
                if self.is_bomb_or_rupoor(i - self.board_width) {
                    neighbors += 1;
                }
                if self.is_bomb_or_rupoor(i + self.board_width) {
                    neighbors += 1;
                }

                // give content based on neighbors
                if neighbors == 0 {
                    self.holes[i as usize] = HoleContent::GreenRupee;
                } else if neighbors < 3 {
                    self.holes[i as usize] = HoleContent::BlueRupee;
                } else if neighbors < 5 {
                    self.holes[i as usize] = HoleContent::RedRupee;
                } else if neighbors < 7 {
                    self.holes[i as usize] = HoleContent::SilverRupee;
                } else if neighbors < 9 {
                    self.holes[i as usize] = HoleContent::GoldRupee;
                }
                // more isn't possible
            }
        }
    }

    fn is_bomb_or_rupoor(&mut self, index: isize) -> bool {
        if (-1 < index) && (index < self.hole_count) {
            let content = &self.holes[index as usize];
            return *content == HoleContent::Rupoor || *content == HoleContent::Bomb;
        }
        return false;
    }

    fn draw(&self) {
        // for h in self.holes.iter() {
        //     h.draw();
        // }
        for i in 0..(self.board_height) {
            for j in 0..(self.board_width) {
                self.holes[(i * self.board_width + j) as usize].draw();
            }
            println!();
        }
    }
}