use std::{
    fmt,
    io::{self, BufRead},
};

struct OctMap(Vec<Vec<u32>>);
impl OctMap {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_valid_index(&self, x: i32, y: i32) -> bool {
        let max_x = self.0.len() as i32 - 1;
        let max_y = self.0[0].len() as i32 - 1;

        if x < 0 || x > max_x || y < 0 || y > max_y {
            return false;
        }

        true
    }
    pub fn get(&self, x: i32, y: i32) -> u32 {
        if !self.is_valid_index(x, y) {
            return 0;
        }

        self.0[x as usize][y as usize]
    }
    pub fn add(&mut self, x: i32, y: i32, num: u32) {
        if !self.is_valid_index(x, y) {
            return;
        }

        self.0[x as usize][y as usize] += num;
    }
}
impl fmt::Display for OctMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "".to_owned();

        for x in 0..self.0.len() {
            str.push_str(&self.0[x].iter().map(|x| x.to_string()).collect::<String>());
            str.push_str("\n");
        }

        write!(f, "{}", str)
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut map = OctMap::new();
    for _line in lines {
        let line = _line.unwrap();
        let row = line
            .chars()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        map.0.push(row);
    }

    let max_x = map.0.len() - 1;
    let max_y = map.0[0].len() - 1;

    let mut steps = 0;
    loop {
        steps += 1;

        for x in 0..=max_x {
            for y in 0..=max_y {
                map.add(x as i32, y as i32, 1);
            }
        }

        loop {
            let mut flashed = false;

            for x in 0..=max_x {
                for y in 0..=max_y {
                    if map.0[x][y] > 9 {
                        flashed = true;

                        map.0[x][y] = 0;
                        for dx in x as i32 - 1..=x as i32 + 1 {
                            for dy in y as i32 - 1..=y as i32 + 1 {
                                if !map.is_valid_index(dx, dy) {
                                    continue;
                                }
                                if map.get(dx, dy) == 0 {
                                    continue;
                                }
                                map.add(dx, dy, 1);
                            }
                        }
                    }
                }
            }

            if !flashed {
                break;
            }
        }

        let all_flashed = map
            .0
            .iter()
            .map(|row| row.iter().map(|x| x == &0).fold(true, |x, y| x && y))
            .fold(true, |x, y| x && y);
        if all_flashed {
            break;
        }
    }

    println!("ans {}", steps);
}
