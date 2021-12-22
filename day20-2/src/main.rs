use std::{
    collections::HashSet,
    fmt,
    io::{self, BufRead},
};

use rusttype::Point as RPoint;

type Filter = Vec<bool>;

type Point = RPoint<i32>;
struct Map {
    pub hashset: HashSet<Point>,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub outside_cell_state: bool,
}
impl Map {
    pub fn new() -> Self {
        Self {
            hashset: HashSet::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            outside_cell_state: false,
        }
    }

    pub fn insert(&mut self, p: Point) {
        self.min_x = if p.x < self.min_x { p.x } else { self.min_x };
        self.max_x = if p.x > self.max_x { p.x } else { self.max_x };
        self.min_y = if p.y < self.min_y { p.y } else { self.min_y };
        self.max_y = if p.y > self.max_y { p.y } else { self.max_y };
        self.hashset.insert(p);
    }

    pub fn calc_filter_index(&self, p: Point) -> usize {
        let mut bits: Vec<char> = vec![];
        for y in p.y - 1..=p.y + 1 {
            for x in p.x - 1..=p.x + 1 {
                if x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y {
                    bits.push(if self.outside_cell_state { '1' } else { '0' });
                    continue;
                }

                bits.push(if self.hashset.contains(&Point { x, y }) {
                    '1'
                } else {
                    '0'
                })
            }
        }

        usize::from_str_radix(&bits.iter().collect::<String>(), 2).unwrap()
    }

    pub fn apply_filter(&self, filter: &Filter) -> Self {
        let mut new_map = Map::new();

        for y in self.min_y - 1..=self.max_y + 1 {
            for x in self.min_x - 1..=self.max_x + 1 {
                let point = Point { x, y };
                let i = self.calc_filter_index(point);
                if filter[i] {
                    new_map.insert(point);
                }
            }
        }

        new_map.outside_cell_state = if self.outside_cell_state {
            filter[511]
        } else {
            filter[0]
        };

        new_map
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "".to_owned();

        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                str.push(if self.hashset.contains(&Point { x, y }) {
                    '#'
                } else {
                    '.'
                });
            }
            str.push('\n');
        }

        write!(f, "{}", str)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut filter = Filter::new();
    for c in lines.next().unwrap().unwrap().chars() {
        filter.push(c == '#');
    }

    lines.next();

    let mut map = Map::new();
    for (_line, y) in lines.zip(0..) {
        let line = _line.unwrap();
        for (c, x) in line.chars().zip(0..) {
            if c == '#' {
                map.insert(Point { x, y });
            }
        }
    }

    let mut filterd_map = map;
    for _ in 0..50 {
        filterd_map = filterd_map.apply_filter(&filter);
    }
    println!("ans {}", filterd_map.hashset.len());
}
