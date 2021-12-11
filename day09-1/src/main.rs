use std::io::{self, BufRead};

type Cell = u8;
type Map = Vec<Vec<Cell>>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut map: Map = Vec::new();
    for _line in lines {
        let line = _line.unwrap();
        let map_row = line
            .chars()
            .map(|x| x.to_string().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        map.push(map_row);
    }

    let max_x = map.len();
    let max_y = map[0].len();
    let mut ans: i32 = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if x != 0 && map[x - 1][y] <= map[x][y] {
                continue;
            }
            if y != 0 && map[x][y - 1] <= map[x][y] {
                continue;
            }
            if x != max_x - 1 && map[x + 1][y] <= map[x][y] {
                continue;
            }
            if y != max_y - 1 && map[x][y + 1] <= map[x][y] {
                continue;
            }

            ans += map[x][y] as i32 + 1;
        }
    }

    println!("ans {}", ans);
}
