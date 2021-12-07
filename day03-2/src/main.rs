use std::io::{self, BufRead};

fn get_digit_count(report: &Vec<String>, digit_index: usize) -> i32 {
    let mut count = 0;

    for line in report {
        count += match line.chars().nth(digit_index).unwrap() {
            '0' => -1,
            '1' => 1,
            _ => 0,
        };
    }

    count
}

fn filter_report(report: &Vec<String>, nth: usize, char: char) -> Vec<String> {
    let filterd: Vec<String> = report
        .iter()
        .cloned()
        .filter(|line| line.chars().nth(nth).unwrap() == char)
        .collect();

    filterd
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut report: Vec<String> = Vec::new();
    for _line in lines {
        report.push(_line.unwrap());
    }

    let line_len = report.first().unwrap().len();

    let mut oxygen_generator_rating_report = report.clone();
    for i in 0..line_len {
        let digit_cnt = get_digit_count(&oxygen_generator_rating_report, i);
        let filter_char = if digit_cnt == 0 {
            '1'
        } else if digit_cnt > 0 {
            '1'
        } else {
            '0'
        };
        oxygen_generator_rating_report =
            filter_report(&oxygen_generator_rating_report, i, filter_char);

        if oxygen_generator_rating_report.len() < 2 {
            break;
        }
    }

    let mut co2_scrubber_rating_report = report.clone();
    for i in 0..line_len {
        let digit_cnt = get_digit_count(&co2_scrubber_rating_report, i);
        let filter_char = if digit_cnt == 0 {
            '0'
        } else if digit_cnt > 0 {
            '0'
        } else {
            '1'
        };
        co2_scrubber_rating_report = filter_report(&co2_scrubber_rating_report, i, filter_char);

        if co2_scrubber_rating_report.len() < 2 {
            break;
        }
    }

    println!(
        "{:?} {:?}",
        oxygen_generator_rating_report, co2_scrubber_rating_report
    );

    let oxygen_generator_rating =
        i32::from_str_radix(oxygen_generator_rating_report.first().unwrap(), 2).unwrap();
    let co2_scrubber_rating =
        i32::from_str_radix(co2_scrubber_rating_report.first().unwrap(), 2).unwrap();

    println!("ans {}", oxygen_generator_rating * co2_scrubber_rating);
}
