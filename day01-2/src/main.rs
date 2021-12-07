use std::io::{self, BufRead};

const WINDOW_SIZE: i32 = 3;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut window: Vec<i32> = Vec::new();
    for _ in 0..WINDOW_SIZE {
        window.push(lines.next().unwrap().unwrap().parse::<i32>().unwrap());
    }

    let mut increase_cnt = 0;
    for line in lines {
        let value: i32 = line.unwrap().parse().unwrap();

        window.push(value);
        let removed = window.remove(0);
        if removed < value {
            increase_cnt += 1;
        }
    }

    println!("increase cnt: {}", increase_cnt);
}
