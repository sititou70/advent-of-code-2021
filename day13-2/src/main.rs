use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use rusttype::Point;

type Paper = HashSet<Point<i32>>;

#[derive(Debug)]
struct Fold {
    pub direction: char,
    position: i32,
}

fn fold(paper: &Paper, fold: &Fold) -> Paper {
    let mut new_paper = Paper::new();

    if fold.direction == 'x' {
        for pos in paper {
            new_paper.insert(Point {
                x: if pos.x > fold.position {
                    fold.position * 2 - pos.x
                } else {
                    pos.x
                },
                y: pos.y,
            });
        }
    }
    if fold.direction == 'y' {
        for pos in paper {
            new_paper.insert(Point {
                x: pos.x,
                y: if pos.y > fold.position {
                    fold.position * 2 - pos.y
                } else {
                    pos.y
                },
            });
        }
    }

    new_paper
}

fn print_paper(paper: &Paper) {
    let mut max_x = 0;
    let mut max_y = 0;

    for point in paper {
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin_str = String::from_utf8(stdin.lock().fill_buf().unwrap().to_vec()).unwrap();
    let mut splited = stdin_str.split("\n\n");

    let mut paper = Paper::new();
    for line in splited.next().unwrap().split("\n") {
        let parsed = line
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        paper.insert(Point {
            x: parsed[0],
            y: parsed[1],
        });
    }

    let mut folds: Vec<Fold> = Vec::new();
    for line in splited.next().unwrap().split("\n") {
        let splited = line.split(" ").collect::<Vec<&str>>()[2]
            .split("=")
            .collect::<Vec<&str>>();

        folds.push(Fold {
            direction: splited[0].chars().next().unwrap(),
            position: splited[1].parse::<i32>().unwrap(),
        });
    }

    let mut folded = paper;
    for item in folds {
        folded = fold(&folded, &item);
    }

    print_paper(&folded);
}
