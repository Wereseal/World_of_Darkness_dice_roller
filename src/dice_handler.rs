use rand::{Rng, rngs::ThreadRng};
use crate::io::Args;
pub struct DiceState {
    total: i32,
    roll: i32,
    rand: ThreadRng,
}
impl DiceState {
    fn new() -> Self {
        DiceState {
            total: 0,
            rand: rand::rng(),
            roll: 0,
        }
    }
    fn roll(&mut self) {
        self.roll = self.rand.random_range(1..11);
        print!("{} ", self.roll);
    }
    fn sum_roll (&mut self, args: &mut Args) {
        self.roll();
        self.total += self.roll;
        args.remaining_rolls -= 1;
    }
    fn regular_roll(&mut self, args: &mut Args) {
        self.roll();
        match self.roll {
            1 => self.total -= 1,
            2..10 => self.total += if self.roll >= args.difficulty.expect("Difficulty was none.") as i32 {1} else {0},
            10 => args.remaining_rolls += 2,
            _ => panic!("Rolled outside of range"),
        }
        args.remaining_rolls -=1;
        
    }
    fn special_roll(&mut self, args: &mut Args) {
        self.roll();
        match self.roll {
            1 => self.total -= 1,
            2..10 => self.total += if self.roll >= args.difficulty.expect("Difficulty was none.") as i32 {1} else {0},
            10 => self.total += 2,
            _ => panic!("Rolled outside of range"),
        }
        args.remaining_rolls -= 1;
    }
    pub fn roll_all(mut args: Args) -> i32 {
        println!("rolling...\n");
        let mut state: DiceState = DiceState::new();
        if args.difficulty == None {
            for _ in 0..args.remaining_rolls {
                state.sum_roll(&mut args);
            }
        } else if args.is_special {
            while args.remaining_rolls > 0 {
                state.special_roll(&mut args);
            }
        } else {
            while args.remaining_rolls > 0 {
                state.regular_roll(&mut args);
            }
        }
        state.total
    }
}
