use std::{
    cmp::*,
    io::{self, BufRead},
};

const HALLWAY_LENGTH: usize = 11;
const ROOMS_NUM: usize = 4;
const ROOM_DEPTH: usize = 2;

type Cost = u32;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Map {
    hallway: [char; HALLWAY_LENGTH],
    rooms: [Vec<char>; ROOMS_NUM],
}
impl Map {
    pub fn new() -> Self {
        Map {
            hallway: ['\0'; HALLWAY_LENGTH],
            rooms: [vec![], vec![], vec![], vec![]],
        }
    }

    pub fn search_min_cost(&self, current_cost: Cost, limit_cost: &mut Cost) -> Option<Cost> {
        if self.is_solved() {
            *limit_cost = min(*limit_cost, current_cost);
            return Some(0);
        }

        if current_cost > *limit_cost {
            return None;
        }

        let mut cost: Cost = Cost::MAX;
        let mut moved = false;
        for (next_map, next_cost) in self.get_next_maps() {
            if let Some(after_cost) = next_map.search_min_cost(current_cost + next_cost, limit_cost)
            {
                let total_cost = next_cost + after_cost;
                cost = min(cost, total_cost);
                moved = true;
            }
        }
        if !moved {
            return None;
        }

        Some(cost)
    }

    pub fn get_next_maps(&self) -> Vec<(Map, Cost)> {
        // 正しい位置に入れられるitemがあればいれる
        for (hallway_index, char) in self.hallway.iter().enumerate() {
            if *char == '\0' {
                continue;
            }

            let target_room_index = self.get_rooms_index(*char);
            if !self.is_pushable_room(target_room_index) {
                continue;
            }

            let target_hallway_index = self.get_rooms_hallway_index(target_room_index);
            let trajectory_indexes = if hallway_index < target_hallway_index {
                hallway_index + 1..target_hallway_index
            } else {
                target_hallway_index + 1..hallway_index
            };
            let is_pushable = self.hallway[trajectory_indexes]
                .iter()
                .map(|x| *x == '\0')
                .fold(true, |x, y| x && y);
            if !is_pushable {
                continue;
            }

            // pushする
            let mut new_map = self.clone();
            new_map.hallway[hallway_index] = '\0';
            new_map.rooms[target_room_index].push(*char);
            let move_num = (target_hallway_index as i32 - hallway_index as i32).abs() as Cost
                + (ROOM_DEPTH as Cost - new_map.rooms[target_room_index].len() as Cost + 1);
            let cost = move_num * self.get_char_weight(*char);
            return vec![(new_map, cost)];
        }

        // 取り出すべきitemがあればあらゆる位置に取り出す
        let mut poped_maps: Vec<(Map, Cost)> = vec![];
        for room_index in 0..ROOMS_NUM {
            if self.rooms[room_index].len() == 0 {
                continue;
            }
            if self.is_no_outsider_room(room_index) {
                continue;
            }

            let mut popable_hallway_indexes: Vec<usize> = vec![];
            let room_hallway_index = self.get_rooms_hallway_index(room_index);
            // 右に走査する
            for i in room_hallway_index + 1..HALLWAY_LENGTH {
                if self.hallway[i] != '\0' {
                    break;
                }
                if self.is_room_entrance_index(i) {
                    continue;
                }

                popable_hallway_indexes.push(i);
            }
            // 左に走査する
            for i in (0..=room_hallway_index - 1).rev() {
                if self.hallway[i] != '\0' {
                    break;
                }
                if self.is_room_entrance_index(i) {
                    continue;
                }

                popable_hallway_indexes.push(i);
            }
            if popable_hallway_indexes.len() == 0 {
                continue;
            }

            let pop_moves = ROOM_DEPTH - self.rooms[room_index].len() + 1;
            for popable_hallway_index in popable_hallway_indexes {
                let mut new_map = self.clone();
                let item = new_map.rooms[room_index].pop().unwrap();
                new_map.hallway[popable_hallway_index] = item;

                let moves = (popable_hallway_index as i32 - room_hallway_index as i32).abs()
                    as Cost
                    + pop_moves as Cost;
                let cost = moves * self.get_char_weight(item);

                poped_maps.push((new_map, cost));
            }
        }
        if poped_maps.len() != 0 {
            return poped_maps;
        }

        // 打つ手なし
        vec![]
    }

    pub fn get_char_weight(&self, char: char) -> Cost {
        (10 as Cost).pow(self.get_rooms_index(char) as Cost)
    }
    pub fn get_rooms_char(&self, room_index: u32) -> char {
        std::char::from_u32('A' as u32 + room_index).unwrap()
    }
    pub fn get_rooms_index(&self, room_char: char) -> usize {
        room_char as usize - 'A' as usize
    }
    pub fn get_rooms_hallway_index(&self, room_index: usize) -> usize {
        (room_index + 1) * 2
    }
    pub fn is_room_entrance_index(&self, hallway_index: usize) -> bool {
        hallway_index % 2 == 0
            && hallway_index >= self.get_rooms_hallway_index(0)
            && hallway_index <= self.get_rooms_hallway_index(ROOMS_NUM - 1)
    }

    pub fn is_no_outsider_room(&self, room_index: usize) -> bool {
        let room_char = self.get_rooms_char(room_index as u32);
        self.rooms[room_index]
            .iter()
            .map(|x| *x == room_char)
            .fold(true, |x, y| x && y)
    }
    pub fn is_pushable_room(&self, room_index: usize) -> bool {
        self.rooms[room_index].len() < ROOM_DEPTH && self.is_no_outsider_room(room_index)
    }

    pub fn is_solved(&self) -> bool {
        for (i, room) in self.rooms.iter().enumerate() {
            if room.len() != ROOM_DEPTH {
                return false;
            }
            let room_char = self.get_rooms_char(i as u32);
            for char in room {
                if *char != room_char {
                    return false;
                }
            }
        }

        true
    }

    pub fn print(&self) {
        for c in self.hallway {
            print!("{}", if c == '\0' { '.' } else { c });
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn test_get_rooms_char() {
        let map = Map::new();
        assert_eq!(map.get_rooms_char(0), 'A');
        assert_eq!(map.get_rooms_char(2), 'C');
        assert_eq!(map.get_rooms_char(4), 'E');
        assert_eq!(map.get_rooms_char(6), 'G');
    }

    #[test]
    fn test_pushable_next_map() {
        let mut map = Map::new();
        map.hallway[0] = 'D';
        let moved = map.get_next_maps();
        assert_eq!(moved.len(), 1);
        assert_eq!(moved[0].1, 10000);
    }
    #[test]
    fn test_popable_next_map() {
        let mut map = Map::new();
        map.hallway[0] = 'D';
        map.rooms[3].push('A');
        let moved = map.get_next_maps();
        assert_eq!(moved.len(), 6);
        assert_eq!(moved[0].1, 3);
    }
    #[test]
    fn test_unsolveable_next_map() {
        let mut map = Map::new();
        map.rooms[0].push('A');
        map.rooms[3].push('D');
        let moved = map.get_next_maps();
        assert_eq!(moved.len(), 0);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // parse map
    lines.next();
    lines.next();
    let mut map = Map::new();
    let mut target_lines: Vec<String> = vec![];
    for _ in 0..ROOM_DEPTH {
        target_lines.push(lines.next().unwrap().unwrap());
    }
    for line in target_lines.iter().rev() {
        let replaced = line.replace("##", "").replace("  ", "");
        let items = replaced.split("#").collect::<Vec<&str>>();
        for i in 0..ROOMS_NUM {
            map.rooms[i].push(items[i + 1].chars().next().unwrap());
        }
    }

    let mut limit_cost = Cost::MAX;
    println!("{:?}", map.search_min_cost(0, &mut limit_cost,));
}
