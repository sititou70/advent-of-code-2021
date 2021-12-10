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
    let mut rightdown_delta: Map = vec![vec![0; MAP_SIZE]; MAP_SIZE];
    let mut rightup_delta: Map = vec![vec![0; MAP_SIZE]; MAP_SIZE];
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

        // horizontal
        if point1[1] == point2[1] {
            if point1[0] < point2[0] {
                horizontal_delta[point1[0]][point1[1]] += 1;
                horizontal_delta[point2[0] + 1][point2[1]] += -1;
            } else {
                horizontal_delta[point2[0]][point2[1]] += 1;
                horizontal_delta[point1[0] + 1][point1[1]] += -1;
            }
            continue;
        }
        // vertical
        if point1[0] == point2[0] {
            if point1[1] < point2[1] {
                vertical_delta[point1[0]][point1[1]] += 1;
                vertical_delta[point2[0]][point2[1] + 1] += -1;
            } else {
                vertical_delta[point2[0]][point2[1]] += 1;
                vertical_delta[point1[0]][point1[1] + 1] += -1;
            }
            continue;
        }
        let gradient =
            (point2[1] as i32 - point1[1] as i32) / (point2[0] as i32 - point1[0] as i32);
        // rightdown
        if gradient > 0 {
            if point1[0] < point2[0] {
                rightdown_delta[point1[0]][point1[1]] += 1;
                rightdown_delta[point2[0] + 1][point2[1] + 1] += -1;
            } else {
                rightdown_delta[point2[0]][point2[1]] += 1;
                rightdown_delta[point1[0] + 1][point1[1] + 1] += -1;
            }
            continue;
        }
        // rightup
        if gradient < 0 {
            if point1[0] < point2[0] {
                rightup_delta[point1[0]][point1[1]] += 1;
                if point2[1] == 0 {
                    continue;
                };
                rightup_delta[point2[0] + 1][point2[1] - 1] += -1;
            } else {
                rightup_delta[point2[0]][point2[1]] += 1;
                if point1[1] == 0 {
                    continue;
                };
                rightup_delta[point1[0] + 1][point1[1] - 1] += -1;
            }
            continue;
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
    //// rightdown
    let mut start_points: Vec<[usize; 2]> = Vec::new();
    start_points.push([0, 0]);
    start_points.append(&mut (1..MAP_SIZE).map(|x| [x, 0]).collect::<Vec<[usize; 2]>>());
    start_points.append(&mut (1..MAP_SIZE).map(|x| [0, x]).collect::<Vec<[usize; 2]>>());
    for start_point in start_points {
        let mut x = start_point[0];
        let mut y = start_point[1];

        let mut delta = 0;
        while x < MAP_SIZE && y < MAP_SIZE {
            delta += rightdown_delta[x][y];
            map[x][y] += delta;

            x += 1;
            y += 1;
        }
    }
    //// rightup
    let mut start_points: Vec<[usize; 2]> = Vec::new();
    start_points.append(&mut (0..MAP_SIZE).map(|x| [0, x]).collect::<Vec<[usize; 2]>>());
    start_points.append(
        &mut (1..MAP_SIZE)
            .map(|x| [x, MAP_SIZE - 1])
            .collect::<Vec<[usize; 2]>>(),
    );
    for start_point in start_points {
        let mut x = start_point[0];
        let mut y = start_point[1];

        let mut delta = 0;
        while x < MAP_SIZE {
            delta += rightup_delta[x][y];
            map[x][y] += delta;

            x += 1;
            if y == 0 {
                break;
            }
            y -= 1;
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
