use std::io::{self, BufRead};

use std::collections::HashMap;

const WINS_SCORE: u32 = 21;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct PlayerState {
    pub pos: u32, // 1 ~ 10
    pub score: u32,
}
// (current player state, next player state)
type GameState = (PlayerState, PlayerState);

// (p1 wins, p2 wins)
type Wins = (u128, u128);
type CalcWinsCache = HashMap<GameState, Wins>;
fn calc_wins(
    current_player: PlayerState,
    next_player: PlayerState,
    cahce: &mut CalcWinsCache,
) -> Wins {
    if let Some(wins) = cahce.get(&(current_player, next_player)) {
        return *wins;
    };

    if current_player.score >= WINS_SCORE {
        return (1, 0);
    };
    if next_player.score >= WINS_SCORE {
        return (0, 1);
    };

    let mut current_wins: u128 = 0;
    let mut next_wins: u128 = 0;
    for (roll, freq) in vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let pos = (current_player.pos - 1 + roll) % 10 + 1;
        let wins = calc_wins(
            next_player,
            PlayerState {
                pos,
                score: current_player.score + pos,
            },
            cahce,
        );
        current_wins += wins.1 * freq;
        next_wins += wins.0 * freq;
    }

    let wins = (current_wins, next_wins);
    cahce.insert((current_player, next_player), wins);
    wins
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let p1_pos = lines
        .next()
        .unwrap()
        .unwrap()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
    let p2_pos = lines
        .next()
        .unwrap()
        .unwrap()
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let mut cahce = CalcWinsCache::new();
    let wins = calc_wins(
        PlayerState {
            pos: p1_pos,
            score: 0,
        },
        PlayerState {
            pos: p2_pos,
            score: 0,
        },
        &mut cahce,
    );
    println!("ans {:?}", if wins.0 > wins.1 { wins.0 } else { wins.1 });
}
