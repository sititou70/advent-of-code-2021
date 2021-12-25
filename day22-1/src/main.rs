use std::cmp::{max, min};
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use euclid::default::Point3D;

type Range = (i32, i32);
type Point = Point3D<i32>;

const MAP_SIZE: i32 = 50;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut points: HashSet<Point> = HashSet::new();
    for _line in lines {
        let line = _line.unwrap();
        let splited = line.split(" ").collect::<Vec<&str>>();
        let mode = splited[0];
        let ranges = splited[1]
            .replace("x=", "")
            .replace("y=", "")
            .replace("z=", "")
            .split(",")
            .map(|x| {
                let nums = x
                    .split("..")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (nums[0], nums[1])
            })
            .collect::<Vec<Range>>();

        for x in max(ranges[0].0, -MAP_SIZE)..=min(ranges[0].1, MAP_SIZE) {
            for y in max(ranges[1].0, -MAP_SIZE)..=min(ranges[1].1, MAP_SIZE) {
                for z in max(ranges[2].0, -MAP_SIZE)..=min(ranges[2].1, MAP_SIZE) {
                    if mode == "on" {
                        points.insert(Point::new(x, y, z));
                    } else {
                        points.remove(&Point::new(x, y, z));
                    }
                }
            }
        }
    }

    println!("ans {}", points.len());
}
