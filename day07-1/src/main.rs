use std::cmp;
use std::io::{self, BufRead};

fn get_cost(target_position: i32, positions: &Vec<i32>) -> i32 {
    let mut cost: i32 = 0;
    for position in positions {
        cost += (target_position - position).abs();
    }
    cost
}

fn main() {
    let stdin = io::stdin();

    // initial state of fish: [0 timer fish, 1 timer fish, ...]
    let positions = String::from_utf8(stdin.lock().fill_buf().unwrap().to_vec())
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let max_position = positions.iter().max().unwrap();

    let mut min_cost = max_position * positions.len() as i32;
    for target_position in 0..*max_position {
        let cost = get_cost(target_position, &positions);
        min_cost = cmp::min(cost, min_cost);
    }

    println!("ans {}", min_cost);
}
