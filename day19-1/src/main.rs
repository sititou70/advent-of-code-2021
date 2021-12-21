use euclid::default::{Point3D, Vector3D};
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

type Vector = Vector3D<i32>;
type Point = Point3D<i32>;
type Scan = HashSet<Point>;
type Scans = Vec<Scan>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut scans = Scans::new();
    let mut scan = Scan::new();
    for _line in lines {
        let line = _line.unwrap();

        if line == "" {
            continue;
        }
        if line.contains("scanner") {
            if scan.len() != 0 {
                scans.push(scan.clone())
            };
            scan.clear();
            continue;
        }

        let parsed = line
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        scan.insert(Point::new(parsed[0], parsed[1], parsed[2]));
    }
    scans.push(scan);

    let mut unmatched_scans = scans.clone();
    let mut all_points = Scan::new();
    for p in &scans[0] {
        all_points.insert(*p);
    }
    unmatched_scans.remove(0);

    while &unmatched_scans.len() != &0 {
        let mut next_unmatched_scans = Scans::new();
        for scan in &scans {
            if let Some((angle_id, pos)) = match_scan(&all_points, &scan) {
                for p in scan {
                    all_points.insert(rotate(*p, angle_id) + pos);
                }
            } else {
                next_unmatched_scans.push(scan.clone());
            }
        }

        if next_unmatched_scans.len() == unmatched_scans.len() {
            break;
        }

        println!("rest scans: {}", next_unmatched_scans.len());
        unmatched_scans = next_unmatched_scans;
    }

    println!("ans {}", all_points.len());
}

const LEAST_MATCH_POINTS_NUM: usize = 12;
// s1: origin stanner, s2: opponent scanner -> (angle_id of rotate, opponent scanner point)
fn match_scan(s1: &Scan, s2: &Scan) -> Option<(i32, Vector)> {
    for p1 in s1 {
        for angle_id in 0..24 {
            let mut rotated_s2 = Scan::new();
            for p in s2 {
                rotated_s2.insert(rotate(*p, angle_id));
            }

            for p2 in &rotated_s2 {
                let s2_pos = *p1 - *p2;

                let mut s2_set: HashSet<Point> = HashSet::new();
                for p in &rotated_s2 {
                    s2_set.insert(*p + s2_pos);
                }

                if s2_set.intersection(&s1).collect::<Vec<&Point>>().len() >= LEAST_MATCH_POINTS_NUM
                {
                    return Some((angle_id, s2_pos));
                }
            }
        }
    }

    None
}

// angle_id: 0 ~ 23
fn rotate(p: Point, angle_id: i32) -> Point {
    // 0: x, 1: y, 2: z
    let axis_id = angle_id / 8;
    // 0: no flip, 1: flip
    let flip_id = (angle_id / 4) % 2;
    let rotate_angle_id = angle_id % 4;

    let axis_rotated_point = match axis_id {
        0 => p,
        1 => rotate_z(p, 3),
        2 => rotate_y(p, 1),
        _ => panic!("unexpected axis_id: {}", axis_id),
    };

    let flipped_point = if flip_id == 0 {
        axis_rotated_point
    } else {
        match axis_id {
            0 => rotate_y(axis_rotated_point, 2),
            1 => rotate_z(axis_rotated_point, 2),
            2 => rotate_y(axis_rotated_point, 2),
            _ => panic!("unexpected axis_id: {}", axis_id),
        }
    };

    match axis_id {
        0 => rotate_x(flipped_point, rotate_angle_id),
        1 => rotate_y(flipped_point, rotate_angle_id),
        2 => rotate_z(flipped_point, rotate_angle_id),
        _ => panic!("unexpected axis_id: {}", axis_id),
    }
}

// angle_id: 0 ~ 3
fn rotate_x(p: Point, angle_id: i32) -> Point {
    match angle_id {
        0 => p,
        1 => Point::new(p.x, -p.z, p.y),
        2 => Point::new(p.x, -p.y, -p.z),
        3 => Point::new(p.x, p.z, -p.y),
        _ => panic!("unexpected angle_id: {}", angle_id),
    }
}
fn rotate_y(p: Point, angle_id: i32) -> Point {
    match angle_id {
        0 => p,
        1 => Point::new(p.z, p.y, -p.x),
        2 => Point::new(-p.x, p.y, -p.z),
        3 => Point::new(-p.z, p.y, p.x),
        _ => panic!("unexpected angle_id: {}", angle_id),
    }
}
fn rotate_z(p: Point, angle_id: i32) -> Point {
    match angle_id {
        0 => p,
        1 => Point::new(p.y, -p.x, p.z),
        2 => Point::new(-p.x, -p.y, p.z),
        3 => Point::new(-p.y, p.x, p.z),
        _ => panic!("unexpected angle_id: {}", angle_id),
    }
}
