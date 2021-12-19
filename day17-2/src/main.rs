use std::io::{self, BufRead};

use rusttype::Point as RPoint;

type Point = RPoint<i32>;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let aria = line
        .replace("target area: ", "")
        .replace("x=", "")
        .replace(", y=", "..")
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut within_num = 0;
    for init_x in -1000..1000 {
        for init_y in -1000..1000 {
            let within = tobe_within(
                Point { x: 0, y: 0 },
                Point {
                    x: init_x,
                    y: init_y,
                },
                Point {
                    x: aria[0],
                    y: aria[2],
                },
                Point {
                    x: aria[1],
                    y: aria[3],
                },
            );
            if within {
                within_num += 1;
            }
        }
    }

    println!("ans {}", within_num);
}

fn tobe_within(init_pos: Point, init_v: Point, aria_from: Point, aria_to: Point) -> bool {
    let mut pos = init_pos;
    let mut v = init_v;
    loop {
        if aria_from.x <= pos.x && pos.x <= aria_to.x && aria_from.y <= pos.y && pos.y <= aria_to.y
        {
            return true;
        }

        if v.y < 0 && pos.y < aria_from.y {
            return false;
        }

        pos.x += v.x;
        pos.y += v.y;

        v.y -= 1;
        if v.x > 0 {
            v.x -= 1;
        }
        if v.x < 0 {
            v.x += 1;
        }
    }
}
