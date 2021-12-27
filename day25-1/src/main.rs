use rusttype::Point as RPoint;
use std::{
    cmp::*,
    collections::HashMap,
    io::{self, BufRead},
};

type Point = RPoint<usize>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum SeaCucumber {
    EAST,
    SOUTH,
}

#[derive(PartialEq, Eq, Debug)]
struct Map {
    map: HashMap<Point, SeaCucumber>,
    width: usize,
    height: usize,
}
impl Map {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn east_moved(&self) -> Self {
        let mut new_map = Self::new();
        new_map.width = self.width;
        new_map.height = self.height;

        for (point, item) in &self.map {
            if *item != SeaCucumber::EAST {
                new_map.map.insert(*point, *item);
                continue;
            };
            let dest = Point {
                x: (point.x + 1) % self.width,
                y: point.y,
            };
            if self.map.get(&dest).is_some() {
                new_map.map.insert(*point, *item);
            } else {
                new_map.map.insert(dest, *item);
            }
        }

        new_map
    }
    pub fn south_moved(&self) -> Self {
        let mut new_map = Self::new();
        new_map.width = self.width;
        new_map.height = self.height;

        for (point, item) in &self.map {
            if *item != SeaCucumber::SOUTH {
                new_map.map.insert(*point, *item);
                continue;
            };
            let dest = Point {
                x: point.x,
                y: (point.y + 1) % self.height,
            };
            if self.map.get(&dest).is_some() {
                new_map.map.insert(*point, *item);
            } else {
                new_map.map.insert(dest, *item);
            }
        }

        new_map
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(item) = self.map.get(&Point { x, y }) {
                    print!("{}", if *item == SeaCucumber::EAST { ">" } else { "v" });
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut map = Map::new();
    for (y, _line) in lines.enumerate() {
        map.height = y + 1;

        let line = _line.unwrap();
        for (x, c) in line.chars().enumerate() {
            map.width = x + 1;

            if c == '.' {
                continue;
            }

            map.map.insert(
                Point { x, y },
                if c == '>' {
                    SeaCucumber::EAST
                } else {
                    SeaCucumber::SOUTH
                },
            );
        }
    }

    let mut step = 0;
    loop {
        step += 1;
        let next = map.east_moved().south_moved();
        if map == next {
            break;
        }

        map = next;
        map.print();
    }

    println!("ans {}", step);
}
