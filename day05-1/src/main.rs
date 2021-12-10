use std::io::{self, BufRead};

const MAP_SIZE: usize = 1000;

type Map = Vec<Vec<i32>>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    // calc delta
    // see also: https://imoz.jp/algorithms/imos_method.html
    let mut horizontal_delta: Map = vec![vec![0; MAP_SIZE]; MAP_SIZE];
    let mut vertical_delta: Map = vec![vec![0; MAP_SIZE]; MAP_SIZE];
    for _line in lines {
        let line = _line.unwrap();

        let points = line
            .split(" -> ")
            .map(|x| {
                x.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let point1 = &points[0];
        let point2 = &points[1];

        // for horizontal
        if point1[1] == point2[1] {
            if point1[0] < point2[0] {
                horizontal_delta[point1[0]][point1[1]] += 1;
                horizontal_delta[point2[0] + 1][point2[1]] += -1;
            } else {
                horizontal_delta[point2[0]][point2[1]] += 1;
                horizontal_delta[point1[0] + 1][point1[1]] += -1;
            }
        }
        // for vertical
        if point1[0] == point2[0] {
            if point1[1] < point2[1] {
                vertical_delta[point1[0]][point1[1]] += 1;
                vertical_delta[point2[0]][point2[1] + 1] += -1;
            } else {
                vertical_delta[point2[0]][point2[1]] += 1;
                vertical_delta[point1[0]][point1[1] + 1] += -1;
            }
        }
    }

    // imos
    let mut map: Map = vec![vec![0; MAP_SIZE]; MAP_SIZE];
    //// horizontal
    for y in 0..MAP_SIZE {
        let mut delta = 0;
        for x in 0..MAP_SIZE {
            delta += horizontal_delta[x][y];
            map[x][y] += delta;
        }
    }
    //// vertical
    for x in 0..MAP_SIZE {
        let mut delta = 0;
        for y in 0..MAP_SIZE {
            delta += vertical_delta[x][y];
            map[x][y] += delta;
        }
    }

    // count overlap points
    let mut overlap_points_num = 0;
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            if map[x][y] > 1 {
                overlap_points_num += 1;
            }
        }
    }

    println!("ans {}", overlap_points_num);
}
