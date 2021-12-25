use euclid::default::Point3D;
use std::cmp::{max, min};
use std::io::{self, BufRead};

type Point = Point3D<i32>;

#[derive(Clone, Copy, Debug)]
struct Cuboid(Point, Point);
impl Cuboid {
    pub fn aria(self) -> i128 {
        (self.1.x - self.0.x + 1) as i128
            * (self.1.y - self.0.y + 1) as i128
            * (self.1.z - self.0.z + 1) as i128
    }
}

type Map = Vec<Cuboid>;

fn intersect(c1: Cuboid, c2: Cuboid) -> Option<Cuboid> {
    if c1.1.x < c2.0.x
        || c2.1.x < c1.0.x
        || c1.1.y < c2.0.y
        || c2.1.y < c1.0.y
        || c1.1.z < c2.0.z
        || c2.1.z < c1.0.z
    {
        return None;
    }

    Some(Cuboid(
        Point::new(
            max(c1.0.x, c2.0.x),
            max(c1.0.y, c2.0.y),
            max(c1.0.z, c2.0.z),
        ),
        Point::new(
            min(c1.1.x, c2.1.x),
            min(c1.1.y, c2.1.y),
            min(c1.1.z, c2.1.z),
        ),
    ))
}

fn calc_overlap_aria(map: &Map, cuboid: Cuboid) -> i128 {
    let mut aria = 0;

    for (c, i) in map.iter().zip(0..) {
        if let Some(intersection) = intersect(cuboid, *c) {
            aria += intersection.aria() - calc_overlap_aria(&map[i + 1..].to_vec(), intersection);
        }
    }

    aria
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut instructions: Vec<(String, Cuboid)> = vec![];
    for _line in lines {
        let line = _line.unwrap();
        let splited = line.split(" ").collect::<Vec<&str>>();
        let mode = splited[0];
        let nums = splited[1]
            .replace("x=", "")
            .replace("y=", "")
            .replace("z=", "")
            .replace("..", ",")
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let cuboid = Cuboid(
            Point::new(nums[0], nums[2], nums[4]),
            Point::new(nums[1], nums[3], nums[5]),
        );

        instructions.push((mode.to_string(), cuboid));
    }

    let mut aria: i128 = 0;
    let mut map = Map::new();
    for (mode, cuboid) in instructions.iter().rev() {
        if mode == "on" {
            aria += cuboid.aria() - calc_overlap_aria(&map, *cuboid);
        }

        map.push(*cuboid);
    }

    println!("ans {}", aria);
}
