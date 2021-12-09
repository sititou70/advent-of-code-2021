use std::{
    collections::HashSet,
    io::{self, BufRead},
};

type BoardRow = Vec<i32>;

type Board = Vec<BoardRow>;

fn is_bingo(board: &Board, selected_nums: &HashSet<i32>) -> bool {
    for row in board {
        let bingo = row
            .iter()
            .map(|x| selected_nums.contains(x))
            .fold(true, |x, y| x && y);
        if bingo {
            return bingo;
        }
    }

    let cols_num = board.get(0).unwrap().len();
    for col_index in 0..cols_num {
        let bingo = board
            .iter()
            .map(|row| selected_nums.contains(row.get(col_index).unwrap()))
            .fold(true, |x, y| x && y);

        if bingo {
            return bingo;
        }
    }

    false
}

fn main() {
    let stdin = io::stdin();
    let stdin_str = String::from_utf8(stdin.lock().fill_buf().unwrap().to_vec()).unwrap();
    let mut splited = stdin_str.split("\n\n");

    // parse nums
    let nums = splited
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap());

    // parse boards
    let mut boards: Vec<Board> = Vec::new();
    for board_str in splited {
        let board = board_str
            .split("\n")
            .filter(|line| line != &"")
            .map(|line| {
                line.split(" ")
                    .filter(|x| x != &"")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<BoardRow>()
            })
            .collect::<Board>();
        boards.push(board);
    }

    let mut selected_nums: HashSet<i32> = HashSet::new();
    'num_loop: for num in nums {
        selected_nums.insert(num);
        for board in &boards {
            let bingo = is_bingo(&board, &selected_nums);

            if bingo {
                let rest_sum = board
                    .iter()
                    .map(|row| {
                        row.iter()
                            .filter(|x| !selected_nums.contains(x))
                            .fold(0, |x, y| x + y)
                    })
                    .fold(0, |x, y| x + y);

                println!("ans {}", rest_sum * num);
                break 'num_loop;
            }
        }
    }
}
