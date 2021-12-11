use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use rusttype::Point;

type Cell = u8;
type Map = Vec<Vec<Cell>>;

fn find_basin_cells<'a>(
    point: Point<usize>,
    visited_points: &'a mut HashSet<Point<usize>>,
    map: &'a Map,
) -> &'a mut HashSet<Point<usize>> {
    if visited_points.contains(&point) {
        return visited_points;
    }
    if map[point.x][point.y] == 9 {
        return visited_points;
    }

    let mut points = visited_points;
    points.insert(point);

    if point.x != 0 {
        points = find_basin_cells(
            Point {
                x: point.x - 1,
                y: point.y,
            },
            points,
            map,
        );
    }
    if point.y != 0 {
        points = find_basin_cells(
            Point {
                x: point.x,
                y: point.y - 1,
            },
            points,
            map,
        );
    }
    let max_x = map.len();
    if point.x != max_x - 1 {
        points = find_basin_cells(
            Point {
                x: point.x + 1,
                y: point.y,
            },
            points,
            map,
        );
    }
    let max_y = map[0].len();
    if point.y != max_y - 1 {
        points = find_basin_cells(
            Point {
                x: point.x,
                y: point.y + 1,
            },
            points,
            map,
        );
    }

    points
}

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
    let mut bastin_arias: Vec<usize> = Vec::new();
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

            let mut visited_points: HashSet<Point<usize>> = HashSet::new();
            let points = find_basin_cells(Point { x, y }, &mut visited_points, &map);

            bastin_arias.push(points.len());
        }
    }

    bastin_arias.sort_unstable();
    println!(
        "ans {}",
        bastin_arias.pop().unwrap() * bastin_arias.pop().unwrap() * bastin_arias.pop().unwrap()
    );
}
