use std::cmp;
use std::io::{self, BufRead};

fn get_cost(target_position: i32, positions: &Vec<i32>) -> i32 {
    let mut cost: i32 = 0;
    for position in positions {
        let move_num = (target_position - position).abs();
        cost += move_num * (move_num + 1) / 2;
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

    let mut min_cost = i32::MAX;
    for target_position in 0..*max_position {
        let cost = get_cost(target_position, &positions);
        min_cost = cmp::min(cost, min_cost);
    }

    println!("ans {}", min_cost);
}
