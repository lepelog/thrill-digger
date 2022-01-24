use crate::rng::*;

#[derive(PartialEq, Clone, Copy, FromPrimitive, ToPrimitive, Debug)]
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

    pub fn rupees(&self) -> isize {
        match self {
            HoleContent::Unspecified => 0,
            HoleContent::GreenRupee => 1,
            HoleContent::BlueRupee => 5,
            HoleContent::RedRupee => 20,
            HoleContent::SilverRupee => 100,
            HoleContent::GoldRupee => 300,
            HoleContent::Rupoor => -10,
            HoleContent::Bomb => 0,
        }
    }
}

pub struct HoleMinigame<
    const BOARD_HEIGHT: isize,
    const BOARD_WIDTH: isize,
    const HOLE_COUNT: usize,
    const RUPOOR_COUNT: isize,
    const BOMB_COUNT: isize,
> {}

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

pub type BeginnerHoleMinigame = HoleMinigame<4, 5, 20, 4, 0>;
pub type IntermediateHoleMinigame = HoleMinigame<5, 6, 30, 4, 4>;
pub type ExpertHoleMinigame = HoleMinigame<5, 8, 40, 8, 8>;

impl<
        const BOARD_HEIGHT: isize,
        const BOARD_WIDTH: isize,
        const HOLE_COUNT: usize,
        const RUPOOR_COUNT: isize,
        const BOMB_COUNT: isize,
    > HoleMinigame<BOARD_HEIGHT, BOARD_WIDTH, HOLE_COUNT, RUPOOR_COUNT, BOMB_COUNT>
{
    pub fn generate(rng: &mut RngContext, holes: &mut [HoleContent; HOLE_COUNT]) {
        Self::try_place(rng, holes, 0);
    }

    fn try_place(
        rng: &mut RngContext,
        holes: &mut [HoleContent; HOLE_COUNT],
        recursion_depth: usize,
    ) {
        for hole in holes.iter_mut() {
            *hole = HoleContent::Unspecified;
        }
        Self::place_bombs(rng, holes);
        Self::place_rupoors(rng, holes);
        let mut bad_pattern = false;
        // check if there is a 2x2 group of "bad stuff" aka bombs and rupoors
        'outer: for i in 0..(BOARD_HEIGHT - 1) {
            for j in 0..(BOARD_WIDTH - 1) {
                if Self::is_bomb_or_rupoor(i * BOARD_WIDTH + j, holes)
                    && Self::is_bomb_or_rupoor(i * BOARD_WIDTH + j + 1, holes)
                    && Self::is_bomb_or_rupoor((i + 1) * BOARD_WIDTH + j, holes)
                    && Self::is_bomb_or_rupoor((i + 1) * BOARD_WIDTH + j + 1, holes)
                {
                    bad_pattern = true;
                    break 'outer;
                }
            }
        }
        if bad_pattern && recursion_depth < 10 {
            Self::try_place(rng, holes, recursion_depth + 1);
        } else {
            Self::place_rupees(holes);
        }
    }

    fn place_bombs(rng: &mut RngContext, holes: &mut [HoleContent; HOLE_COUNT]) {
        let mut placed_bombs = 0;
        while placed_bombs < BOMB_COUNT {
            // pick a random field and then check if it's free
            let next_field = (rng.next_float() * HOLE_COUNT as f32) as usize;
            if holes[next_field] == HoleContent::Unspecified {
                holes[next_field] = HoleContent::Bomb;
                placed_bombs += 1;
            }
        }
    }

    fn place_rupoors(rng: &mut RngContext, holes: &mut [HoleContent; HOLE_COUNT]) {
        let mut placed_rupoors = 0;
        while placed_rupoors < RUPOOR_COUNT {
            let next_field = (rng.next_float() * HOLE_COUNT as f32) as usize;
            if holes[next_field] == HoleContent::Unspecified {
                holes[next_field] = HoleContent::Rupoor;
                placed_rupoors += 1;
            }
        }
    }

    fn place_rupees(holes: &mut [HoleContent; HOLE_COUNT]) {
        for i in 0..HOLE_COUNT as isize {
            if !Self::is_bomb_or_rupoor(i, holes) {
                // count neighbors...
                let mut neighbors = 0;
                // check if left side
                if i % BOARD_WIDTH != 0 {
                    if Self::is_bomb_or_rupoor(i - BOARD_WIDTH - 1, holes) {
                        neighbors += 1;
                    }
                    if Self::is_bomb_or_rupoor(i - 1, holes) {
                        neighbors += 1;
                    }
                    if Self::is_bomb_or_rupoor(i + BOARD_WIDTH - 1, holes) {
                        neighbors += 1;
                    }
                }
                // check if right side
                if i % BOARD_WIDTH != (BOARD_WIDTH - 1) {
                    if Self::is_bomb_or_rupoor(i - BOARD_WIDTH + 1, holes) {
                        neighbors += 1;
                    }
                    if Self::is_bomb_or_rupoor(i + 1, holes) {
                        neighbors += 1;
                    }
                    if Self::is_bomb_or_rupoor(i + BOARD_WIDTH + 1, holes) {
                        neighbors += 1;
                    }
                }
                if Self::is_bomb_or_rupoor(i - BOARD_WIDTH, holes) {
                    neighbors += 1;
                }
                if Self::is_bomb_or_rupoor(i + BOARD_WIDTH, holes) {
                    neighbors += 1;
                }

                // give content based on neighbors
                if neighbors == 0 {
                    holes[i as usize] = HoleContent::GreenRupee;
                } else if neighbors < 3 {
                    holes[i as usize] = HoleContent::BlueRupee;
                } else if neighbors < 5 {
                    holes[i as usize] = HoleContent::RedRupee;
                } else if neighbors < 7 {
                    holes[i as usize] = HoleContent::SilverRupee;
                } else if neighbors < 9 {
                    holes[i as usize] = HoleContent::GoldRupee;
                }
                // more isn't possible
            }
        }
    }

    #[inline]
    fn is_bomb_or_rupoor(index: isize, holes: &[HoleContent; HOLE_COUNT]) -> bool {
        if (-1 < index) && (index < HOLE_COUNT as isize) {
            let content = &holes[index as usize];
            return *content == HoleContent::Rupoor || *content == HoleContent::Bomb;
        }
        return false;
    }

    pub fn draw(holes: &[HoleContent; HOLE_COUNT]) {
        // for h in self.holes.iter() {
        //     h.draw();
        // }
        let mut result = String::with_capacity((HOLE_COUNT as isize + BOARD_HEIGHT) as usize);
        for i in 0..(BOARD_HEIGHT) {
            for j in 0..(BOARD_WIDTH) {
                result.push(holes[(i * BOARD_WIDTH + j) as usize].draw());
            }
            result.push('\n');
        }
        println!("{}", result);
    }

    pub fn check_equals(expected: &Vec<HoleContent>, holes: &[HoleContent; HOLE_COUNT]) -> bool {
        holes
            .iter()
            .zip(expected)
            .all(|(is, should)| *should == HoleContent::Unspecified || is == should)
    }

    pub fn get_total_value(holes: &[HoleContent; HOLE_COUNT]) -> isize {
        holes
            .iter()
            .filter(|h| *h != &HoleContent::Rupoor)
            .map(|h| h.rupees())
            .sum()
    }
}

#[cfg(test)]
mod tests {

    use crate::rng::RngContext;
    use crate::thrill_digger::HoleContent::*;
    use crate::thrill_digger::{ExpertHoleMinigame, HoleContent};

    #[test]
    fn check_working() {
        let mut rng = RngContext::from_state(0);
        let mut holes = [HoleContent::Unspecified; 40];
        ExpertHoleMinigame::generate(&mut rng, &mut holes);
        let exp0 = [
            Rupoor,
            RedRupee,
            Rupoor,
            BlueRupee,
            GreenRupee,
            GreenRupee,
            BlueRupee,
            Rupoor,
            RedRupee,
            Rupoor,
            RedRupee,
            RedRupee,
            RedRupee,
            RedRupee,
            RedRupee,
            Bomb,
            RedRupee,
            Rupoor,
            SilverRupee,
            Bomb,
            Bomb,
            Bomb,
            Rupoor,
            RedRupee,
            Bomb,
            Bomb,
            Rupoor,
            Bomb,
            RedRupee,
            SilverRupee,
            Rupoor,
            RedRupee,
            BlueRupee,
            RedRupee,
            RedRupee,
            BlueRupee,
            BlueRupee,
            BlueRupee,
            Bomb,
            BlueRupee,
        ];
        assert_eq!(&exp0, &holes);
        rng.set_state(1);
        ExpertHoleMinigame::generate(&mut rng, &mut holes);
        let exp1 = [
            Rupoor,
            Bomb,
            BlueRupee,
            BlueRupee,
            Bomb,
            Bomb,
            Rupoor,
            RedRupee,
            RedRupee,
            Bomb,
            RedRupee,
            BlueRupee,
            Rupoor,
            SilverRupee,
            Rupoor,
            Bomb,
            RedRupee,
            Rupoor,
            BlueRupee,
            BlueRupee,
            BlueRupee,
            Bomb,
            RedRupee,
            RedRupee,
            Bomb,
            RedRupee,
            BlueRupee,
            BlueRupee,
            BlueRupee,
            RedRupee,
            Bomb,
            BlueRupee,
            BlueRupee,
            BlueRupee,
            Rupoor,
            BlueRupee,
            Rupoor,
            RedRupee,
            Rupoor,
            BlueRupee,
        ];
        assert_eq!(&exp1, &holes);
    }
}
