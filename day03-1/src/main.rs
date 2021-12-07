use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut digit_cnt: Vec<i32> = Vec::new();
    for _line in lines {
        let line = _line.unwrap();
        for (digit, i) in line.chars().rev().zip(0..) {
            match digit_cnt.get(i) {
                None => digit_cnt.push(0),
                _ => (),
            };

            digit_cnt[i] += match digit {
                '0' => -1,
                '1' => 1,
                _ => 0,
            };
        }
    }

    let gamma_str: String = digit_cnt
        .iter()
        .rev()
        .map(|x| if x > &0 { '1' } else { '0' })
        .collect();
    let gamma = i32::from_str_radix(&gamma_str, 2).unwrap();

    let epsilon_str: String = digit_cnt
        .iter()
        .rev()
        .map(|x| if x < &0 { '1' } else { '0' })
        .collect();
    let epsilon = i32::from_str_radix(&epsilon_str, 2).unwrap();

    println!("ans {}", gamma * epsilon);
}
