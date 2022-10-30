use std::ops::Range;

use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng, SeedableRng,
};
use rand_chacha::ChaCha8Rng;
use regex::Regex;

pub struct GameRNG {
    rng_generator: ChaCha8Rng,
    dice_regex: Regex,
}

impl GameRNG {
    pub fn new() -> GameRNG {
        GameRNG {
            rng_generator: rand_chacha::ChaCha8Rng::seed_from_u64(10),
            dice_regex: Regex::new("(\\d+)?d(\\d+)([\\+\\-]\\d+)?").unwrap(),
        }
    }

    pub fn rand_i32(&mut self) -> i32 {
        self.rng_generator.gen()
    }

    pub fn rand_range(&mut self, range: Range<i32>) -> i32 {
        self.rng_generator.gen_range(range)
    }

    pub fn rand_dice(&mut self, dice_str: &str) -> i32 {
        let captures = self.dice_regex.captures(dice_str).unwrap();

        let num_dice: i32 = match captures.get(1) {
            Some(num_dice_match) => num_dice_match.as_str().parse().unwrap(),
            None => 1,
        };

        let dice_sides: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let modifier_opt = captures.get(3).map(|a| a.as_str());

        let mut res_num = 0;

        for _i in 0..num_dice {
            let new_num = self.rng_generator.gen_range(1..=dice_sides);
            res_num += new_num;
        }

        match modifier_opt {
            Some(modifier_str) => {
                let (head, tail) = modifier_str.split_at(1);
                let tail_num: i32 = tail.parse().unwrap();

                if head == "+" {
                    res_num += tail_num;
                } else if head == "-" {
                    res_num -= tail_num;
                }

                res_num
            }
            None => res_num,
        }
    }
}
