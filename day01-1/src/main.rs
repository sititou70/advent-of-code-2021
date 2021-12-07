use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut prev_value: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut increase_cnt = 0;
    for line in lines {
        let value: i32 = line.unwrap().parse().unwrap();
        if value > prev_value {
            increase_cnt += 1;
        }

        prev_value = value;
    }

    println!("increase cnt: {}", increase_cnt);
}
