use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

use rusttype::Point as RPoint;

type Point = RPoint<i32>;
type Points = Vec<Point>;

type Cost = i32;
type CostMap = HashMap<Point, Cost>;

type Map = Vec<Vec<Cost>>;

const EXPAND_MAP_TIMES: i32 = 5;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    // parse input
    let mut map = Map::new();
    for _line in lines {
        let line = _line.unwrap();
        let mut row = line
            .chars()
            .map(|x| x.to_string().parse::<Cost>().unwrap())
            .collect::<Vec<Cost>>();
        let orig_row_width = row.len();

        for times in 0..EXPAND_MAP_TIMES - 1 {
            for i in 0..orig_row_width {
                let cost = row[orig_row_width * times as usize + i];
                row.push(cost % 9 + 1);
            }
        }

        map.push(row);
    }
    let orig_map_height = map.len();
    for times in 0..EXPAND_MAP_TIMES - 1 {
        for i in 0..orig_map_height {
            let row = &map[orig_map_height * times as usize + i];
            let mut new_row: Vec<Cost> = Vec::new();
            for cost in row {
                new_row.push(cost % 9 + 1);
            }

            map.push(new_row);
        }
    }

    let start = Point { x: 0, y: 0 };
    let gorl = Point {
        x: (map[0].len() - 1) as i32,
        y: (map.len() - 1) as i32,
    };

    // A* search
    let mut next_points: HashSet<Point> = HashSet::new();
    let mut estimated_costs = CostMap::new();
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut parents: HashMap<Point, Point> = HashMap::new();
    estimated_costs.insert(start, heuristics(start, &map));
    next_points.insert(start);

    let mut print_cnt = 0;
    loop {
        if next_points.len() == 0 {
            panic!("path not found...");
        }

        let point = *next_points
            .iter()
            .reduce(|x, y| {
                if estimated_costs.get(x).unwrap() < estimated_costs.get(y).unwrap() {
                    x
                } else {
                    y
                }
            })
            .unwrap();
        if point == gorl {
            break;
        }

        next_points.remove(&point);
        visited_points.insert(point);
        let cost = *estimated_costs.get(&point).unwrap();
        let goal_cost = heuristics(point, &map);

        print_cnt += 1;
        if print_cnt % 1000 == 0 {
            println!("heuristics: {}", goal_cost);
        }

        for around_point in get_around_points(point, &map) {
            let around_cost = cost - goal_cost
                + map[around_point.y as usize][around_point.x as usize]
                + heuristics(around_point, &map);

            let visited = visited_points.contains(&around_point);
            let is_next_point = next_points.contains(&around_point);
            if !visited && !is_next_point {
                *estimated_costs.entry(around_point).or_default() = around_cost;
                next_points.insert(around_point);
                parents.insert(around_point, point);
            } else {
                let prev_around_cost = *estimated_costs.get(&around_point).unwrap();

                if around_cost < prev_around_cost {
                    *estimated_costs.entry(around_point).or_default() = around_cost;
                    visited_points.remove(&around_point);
                    next_points.insert(around_point);
                    parents.insert(around_point, point);
                }
            }
        }
    }

    let mut total_cost = 0;
    let mut current_point = gorl;
    while current_point != start {
        total_cost += map[current_point.y as usize][current_point.x as usize];
        current_point = *parents.get(&current_point).unwrap();
    }

    println!("ans {}", total_cost);
}

fn heuristics(point: Point, map: &Map) -> Cost {
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;
    max_x as i32 - point.x + max_y as i32 - point.y
}

fn get_around_points(point: Point, map: &Map) -> Points {
    let mut points = Points::new();
    let x = point.x;
    let y = point.y;
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    if x > 0 {
        points.push(Point { x: x - 1, y: y })
    }
    if y > 0 {
        points.push(Point { x: x, y: y - 1 })
    }
    if x < max_x as i32 {
        points.push(Point { x: x + 1, y: y })
    }
    if y < max_y as i32 {
        points.push(Point { x: x, y: y + 1 })
    }

    points
}
