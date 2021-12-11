use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut ans = 0;
    for _line in lines {
        let line = _line.unwrap();
        let splited = line
            .split(" | ")
            .map(|x| x.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        ans += splited[1]
            .iter()
            .filter(|x| match x.len() {
                2 => true,
                4 => true,
                3 => true,
                7 => true,
                _ => false,
            })
            .collect::<Vec<&&str>>()
            .len();
    }

    println!("ans {}", ans);
}
