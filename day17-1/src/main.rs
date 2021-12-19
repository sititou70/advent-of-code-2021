use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let splited = line
        .replace("target area: ", "")
        .replace("x=", "")
        .replace(", y=", "..")
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut max_y = i32::MIN;
    for init_v in 0..1000 {
        let (within, y) = tobe_within_y(0, init_v, splited[2], splited[3]);
        if within && y > max_y {
            max_y = y;
        }
    }

    println!("ans {}", max_y);
}

fn tobe_within_y(init_pos: i32, init_v: i32, target_from: i32, target_to: i32) -> (bool, i32) {
    let mut pos = init_pos;
    let mut v = init_v;
    let mut max_pos = i32::MIN;
    loop {
        if v < 0 && pos < target_from {
            return (false, max_pos);
        }
        if target_from <= pos && pos <= target_to {
            return (true, max_pos);
        }

        if pos > max_pos {
            max_pos = pos;
        }

        pos += v;
        v -= 1;
    }
}
