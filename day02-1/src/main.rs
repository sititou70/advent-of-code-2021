use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut horizontal_position = 0;
    let mut depth = 0;

    for _line in lines {
        let line = _line.unwrap();
        let splited: Vec<&str> = line.split(' ').collect();
        let direction = splited[0];
        let num = splited[1].parse::<i32>().unwrap();

        match direction {
            "forward" => horizontal_position += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => (),
        };
    }

    println!("ans: {}", horizontal_position * depth)
}
