use crate::rng::*;

#[derive(PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum HoleContent {
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
    pub fn draw(self) -> char {
        match self {
            HoleContent::Unspecified => '?',
            HoleContent::GreenRupee => 'G',
            HoleContent::BlueRupee => 'B',
            HoleContent::RedRupee => 'R',
            HoleContent::SilverRupee => 'S',
            HoleContent::GoldRupee => '3',
            HoleContent::Rupoor => 'P',
            HoleContent::Bomb => 'X',
        }
    }
}

pub struct HoleMinigame {
    hole_count: isize,
    board_height: isize,
    board_width: isize,
    rupoor_count: isize,
    bomb_count: isize,
    holes: [HoleContent; 40], // 40 is the max it can be, but I saw a 0x50 (=80) during init, maybe it's bigger? Doesn't matter
    recursion_depth: u8,
}

#[derive(PartialEq, Clone, Copy)]
pub enum HoleMinigameDifficulty {
    Beginner,
    Intermediate,
    Expert,
}

impl HoleMinigameDifficulty {
    pub fn get_board_width(&self) -> isize {
        match self {
            HoleMinigameDifficulty::Beginner => 5,
            HoleMinigameDifficulty::Intermediate => 6,
            HoleMinigameDifficulty::Expert => 8,
        }
    }

    pub fn get_board_height(&self) -> isize {
        match self {
            HoleMinigameDifficulty::Beginner => 4,
            HoleMinigameDifficulty::Intermediate => 5,
            HoleMinigameDifficulty::Expert => 5,
        }
    }

    pub fn get_hole_count(&self) -> isize {
        return self.get_board_height() * self.get_board_width();
    }

    pub fn get_bomb_count(&self) -> isize {
        match self {
            HoleMinigameDifficulty::Beginner => 4,
            HoleMinigameDifficulty::Intermediate => 4,
            HoleMinigameDifficulty::Expert => 8,
        }
    }

    pub fn get_rupoor_count(&self) -> isize {
        match self {
            HoleMinigameDifficulty::Beginner => 0,
            HoleMinigameDifficulty::Intermediate => 4,
            HoleMinigameDifficulty::Expert => 8,
        }
    }
}

impl HoleMinigame {
    pub fn generate(rng: &mut RngContext, difficulty: HoleMinigameDifficulty) -> Self {
        let mut this =  HoleMinigame{
                    hole_count: difficulty.get_hole_count(),
                    board_height: difficulty.get_board_height(),
                    board_width: difficulty.get_board_width(),
                    rupoor_count: difficulty.get_rupoor_count(),
                    bomb_count: difficulty.get_bomb_count(),
                    holes: [HoleContent::Unspecified; 40],
                    recursion_depth: 0,
                };
        this.try_place(rng);
        return this;
    }

    pub fn new(difficulty: &HoleMinigameDifficulty) -> Self {
        HoleMinigame {
            hole_count: difficulty.get_hole_count(),
            board_height: difficulty.get_board_height(),
            board_width: difficulty.get_board_width(),
            rupoor_count: difficulty.get_rupoor_count(),
            bomb_count: difficulty.get_bomb_count(),
            holes: [HoleContent::Unspecified; 40],
            recursion_depth: 0,
        }
    }

    pub fn regenerate(&mut self, rng: &mut RngContext) {
        self.try_place(rng);
    }

    fn try_place(&mut self, rng: &mut RngContext) {
        for hole in self.holes.iter_mut() {
            *hole = HoleContent::Unspecified;
        }
        self.place_bombs(rng);
        self.place_rupoors(rng);
        let mut bad_pattern = false;
        // check if there is a 2x2 group of "bad stuff" aka bombs and rupoors
        'outer: for i in 0..(self.board_height - 1) {
            for j in 0..(self.board_width - 1) {
                if self.is_bomb_or_rupoor(i * self.board_width + j) &&
                self.is_bomb_or_rupoor(i * self.board_width + j + 1) &&
                self.is_bomb_or_rupoor((i + 1) * self.board_width + j) &&
                self.is_bomb_or_rupoor((i + 1) * self.board_width + j + 1) {
                    bad_pattern = true;
                    break 'outer;
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

    #[inline]
    fn is_bomb_or_rupoor(&mut self, index: isize) -> bool {
        if (-1 < index) && (index < self.hole_count) {
            let content = &self.holes[index as usize];
            return *content == HoleContent::Rupoor || *content == HoleContent::Bomb;
        }
        return false;
    }

    pub fn draw(&self) {
        // for h in self.holes.iter() {
        //     h.draw();
        // }
        // let result = String::with_capacity(self.hole_count+self.board_height);
        // for i in 0..(self.board_height) {
        //     for j in 0..(self.board_width) {
        //         result.push(self.holes[(i * self.board_width + j) as usize].draw());
        //     }
        //     result.push('\n');
        // }
    }

    pub fn check_equals(&self, expected: &Vec<HoleContent>) -> bool {
        self.holes.iter().zip(expected).all(|(is, should)| *should == HoleContent::Unspecified || is == should)
    }

    #[inline]
    pub fn get_holes(&self) -> &[HoleContent; 40] {
        &self.holes
    }
}