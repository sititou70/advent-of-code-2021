use std::io::{self, BufRead};

const BORN_INTERVAL: usize = 7;
const FRY_LENGTH: usize = 2;
const AFTER_DAYS: usize = 256;

fn main() {
    let stdin = io::stdin();

    // initial state of fish: [0 timer fish, 1 timer fish, ...]
    let mut fish_timers = [0 as i128; BORN_INTERVAL];
    String::from_utf8(stdin.lock().fill_buf().unwrap().to_vec())
        .unwrap()
        .split(",")
        .for_each(|x| {
            let i = x.parse::<usize>().unwrap();
            fish_timers[i] += 1;
        });

    let mut fry_timers = [0 as i128; FRY_LENGTH];
    for after_day in 1..=AFTER_DAYS {
        let born_i = (after_day - 1) % BORN_INTERVAL;

        let born_fish_num = fish_timers[born_i];

        // growth fry
        fish_timers[born_i] += fry_timers[0];
        fry_timers[0] = fry_timers[1];

        // born
        fry_timers[1] = born_fish_num;
    }

    let all_fishes_num = fish_timers.iter().fold(0, |x, y| x + y) + fry_timers[0] + fry_timers[1];
    println!("{}", all_fishes_num);
}
