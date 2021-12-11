use std::{
    collections::{btree_set::Intersection, HashMap, HashSet},
    io::{self, BufRead},
};

const SEGUMENTS_NUM: usize = 7;

// DigitMap: 各セグメントに接続されているワイヤの可能性を表す
//  00
// 1  2
//  33
// 4  5
//  66
type DigitMap = Vec<HashSet<char>>;

fn filter_digits_map(available_digits: &Vec<&str>) -> DigitMap {
    // prepare
    let mut digit_map: DigitMap = Vec::new();
    for _ in 0..SEGUMENTS_NUM {
        digit_map.push(
            ['a', 'b', 'c', 'd', 'e', 'f', 'g']
                .iter()
                .cloned()
                .collect::<HashSet<char>>(),
        );
    }

    //// available_digits_map: 各available_digitを集合に変換し，セグメント数（要素数）ごとに整理したもの
    let mut available_digits_map: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();
    for available_digit in available_digits {
        let chars = available_digit.chars().collect::<HashSet<char>>();

        let len = chars.len();
        if !available_digits_map.contains_key(&len) {
            available_digits_map.insert(len, Vec::new());
        }

        let vec = available_digits_map.get_mut(&len).unwrap();
        vec.push(chars);
    }

    // filter
    //// 1（= セグメント数2）の制約
    for i in [2, 5] {
        digit_map[i] = &digit_map[i] & &available_digits_map[&2][0].clone();
    }
    for i in [0, 1, 3, 4, 6] {
        digit_map[i] = &digit_map[i] - &available_digits_map[&2][0];
    }
    //// 4（セグメント数4）の制約
    for i in [1, 2, 3, 5] {
        digit_map[i] = &digit_map[i] & &available_digits_map[&4][0].clone();
    }
    for i in [0, 4, 6] {
        digit_map[i] = &digit_map[i] - &available_digits_map[&4][0];
    }
    //// 7（セグメント数3）の制約
    for i in [0, 2, 5] {
        digit_map[i] = &digit_map[i] & &available_digits_map[&3][0].clone();
    }
    for i in [1, 3, 4, 6] {
        digit_map[i] = &digit_map[i] - &available_digits_map[&3][0];
    }
    //// セグメント数6（0，6，9）の制約
    {
        // 共通部分
        //  xx
        // x
        //
        //    x
        //  xx

        // 非共通部分
        //
        //    x
        //  xx
        // x
        //

        // intersection: 共通部分に現れる，つまり非共通部分には現れないワイヤ
        let intersection = &(&available_digits_map[&6][0] & &available_digits_map[&6][1])
            & &available_digits_map[&6][2];
        for i in [2, 3, 4] {
            digit_map[i] = &digit_map[i] - &intersection;
        }
        // not_intersection: 非共通部分に現れる，つまり共通部分に現れないワイヤ
        let union = &(&available_digits_map[&6][0] | &available_digits_map[&6][1])
            | &available_digits_map[&6][2];
        let not_intersection = &union - &intersection;
        for i in [0, 1, 5, 6] {
            digit_map[i] = &digit_map[i] - &not_intersection;
        }
    }
    //// セグメント数5（2，3，5）の制約
    {
        // 共通部分
        //  xx
        //
        //  xx
        //
        //  xx

        // 非共通部分
        //
        // x  x
        //
        // x  x
        //

        // intersection: 共通部分に現れる，つまり非共通部分には現れないワイヤ
        let intersection = &(&available_digits_map[&5][0] & &available_digits_map[&5][1])
            & &available_digits_map[&5][2];
        for i in [1, 2, 4, 5] {
            digit_map[i] = &digit_map[i] - &intersection;
        }
        // not_intersection: 非共通部分に現れる，つまり共通部分に現れないワイヤ
        let union = &(&available_digits_map[&5][0] | &available_digits_map[&5][1])
            | &available_digits_map[&5][2];
        let not_intersection = &union - &intersection;
        for i in [0, 3, 6] {
            digit_map[i] = &digit_map[i] - &not_intersection;
        }
    }

    digit_map
}

fn parse_output(output_strs: &Vec<&str>, digit_map: DigitMap) -> u32 {
    let base: u32 = 10;

    let mut digitchar_to_position_index: HashMap<char, u32> = HashMap::new();
    for (i, set) in digit_map.iter().enumerate() {
        digitchar_to_position_index.insert(*set.iter().next().unwrap(), i as u32);
    }

    let mut ans = 0;
    for (i, output_str) in output_strs.iter().rev().enumerate() {
        let weight = base.pow(i as u32);
        let code = output_str
            .chars()
            .map(|x| digitchar_to_position_index.get(&x).unwrap())
            .map(|i| (1 << i) as u32)
            .fold(0, |x, y| x | y);

        let num = match code {
            0b1110111 => 0,
            0b0100100 => 1,
            0b1011101 => 2,
            0b1101101 => 3,
            0b0101110 => 4,
            0b1101011 => 5,
            0b1111011 => 6,
            0b0100101 => 7,
            0b1111111 => 8,
            0b1101111 => 9,
            _ => 0,
        };

        ans += num * weight;
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut ans = 0;
    for _line in lines {
        let line = _line.unwrap();
        let splited = line
            .split(" | ")
            .map(|x| x.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let digit_map = filter_digits_map(&splited[0]);
        ans += parse_output(&splited[1], digit_map);
    }

    println!("ans {}", ans);
}

#[cfg(test)]
mod tests {
    use crate::{filter_digits_map, parse_output, SEGUMENTS_NUM};

    #[test]
    fn filter_digit() {
        let available_digits = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];
        let digit_map = filter_digits_map(&available_digits);
        let ans = ['d', 'e', 'a', 'f', 'g', 'b', 'c'];

        for map in &digit_map {
            assert_eq!(map.len(), 1);
        }
        for i in 0..SEGUMENTS_NUM {
            assert!(digit_map[i].contains(&ans[i]));
        }
    }
    #[test]
    fn filter_digit2() {
        let inputs = "eadbcf faceb faecgd gdefabc adc ad adbf gfacbe bceda dcegb
ed acegbfd defb ead dbcfae dbeca caefdg bgecfa dabgc efacb
fda dfbeg cegdab fa edfagcb acgde dagfe abcgdf ceaf dfecag
fdae cebgfa df bdf ebcgdf bcdeafg cfbdae afebc fbadc cbgda
facgb gdefabc decbfa cb cba cgfaeb gafec bgfad agcefd bcge
eacfd caebdfg egfbac gaced abfde cf gafcde adbgce gcfd fec
ecfgd fc fec cdfa defcag degbf dbcgaef eacgdb bfagce adgce
afbed eabgfc cbedag ac egdbfc gacf abcfe becgf aedfbgc cea
ecfgbd agebc dfbae fcedgab adgc gfbcae abcged dc cebad cbd
cadeg gb dgaefbc gbd acbfde gabdfc gbcad afdbc abfg degbfc"
            .split('\n')
            .map(|x| x.split(' ').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        for input in inputs {
            let digit_map = filter_digits_map(&input);
            for map in &digit_map {
                assert_eq!(map.len(), 1);
            }
        }
    }

    #[test]
    fn filter_and_parse() {
        let available_digits = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];
        let digit_map = filter_digits_map(&available_digits);
        let output_strs = vec!["cdfeb", "fcadb", "cdfeb", "cdbaf"];

        let out = parse_output(&output_strs, digit_map);
        assert_eq!(out, 5353);
    }
}
