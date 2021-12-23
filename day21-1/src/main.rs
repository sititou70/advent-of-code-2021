use std::io::{self, BufRead};

const WIN_SCORE: i32 = 1000;

struct Dice(i128);
impl Dice {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn roll(&mut self) -> i32 {
        let ans = self.0 % 100 + 1;
        self.0 += 1;
        ans as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut is_p1_turn = true;
    // 1 ~ 10
    let mut p1_pos = lines
        .next()
        .unwrap()
        .unwrap()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();
    let mut p2_pos = lines
        .next()
        .unwrap()
        .unwrap()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut dice = Dice::new();

    while !(p1_score >= WIN_SCORE || p2_score >= WIN_SCORE) {
        let pos = if is_p1_turn { &mut p1_pos } else { &mut p2_pos };
        let score = if is_p1_turn {
            &mut p1_score
        } else {
            &mut p2_score
        };

        let total_move = dice.roll() + dice.roll() + dice.roll();

        *pos = (*pos + total_move - 1) % 10 + 1;
        *score += *pos;

        is_p1_turn = !is_p1_turn;
    }

    println!(
        "ans {}",
        if p1_score < p2_score {
            p1_score
        } else {
            p2_score
        } as i128
            * dice.0
    );
}
