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

    let folded = fold(&paper, &folds[0]);

    println!("ans {}", folded.len());
}
