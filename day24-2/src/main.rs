// 14 digit
type Id = Vec<i128>;

const INPUT_DIGITS_NUM: usize = 14;

fn eql(x: i128, y: i128) -> i128 {
    if x == y {
        1
    } else {
        0
    }
}

fn orig_calc_z(input: &Id) -> i128 {
    let mut input_index: usize = 0;
    let mut x: i128 = 0;
    let mut y: i128 = 0;
    let mut z: i128 = 0;
    let mut w: i128 = 0;

    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 1; //div z 1
    x += 11; //add x 11
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 1; //add y 1
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 1; //div z 1
    x += 10; //add x 10
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 10; //add y 10
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 1; //div z 1
    x += 13; //add x 13
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 2; //add y 2
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 26; //div z 26
    x += -10; //add x -10
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 5; //add y 5
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 1; //div z 1
    x += 11; //add x 11
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 6; //add y 6
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 1; //div z 1
    x += 11; //add x 11
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 0; //add y 0
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 1; //div z 1
    x += 12; //add x 12
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 16; //add y 16
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 26; //div z 26
    x += -11; //add x -11
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 12; //add y 12
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 26; //div z 26
    x += -7; //add x -7
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 15; //add y 15
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 1; //div z 1
    x += 13; //add x 13
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 7; //add y 7
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 26; //div z 26
    x += -13; //add x -13
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 6; //add y 6
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 26; //div z 26
    x += 0; //add x 0
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 5; //add y 5
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 26; //div z 26
    x += -11; //add x -11
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 6; //add y 6
    y *= x; //mul y x
    z += y; //add z y

    input_index += 1;
    w = input[input_index]; //inp w
    x *= 0; //mul x 0
    x += z; //add x z
    x %= 26; //mod x 26
    z /= 26; //div z 26
    x += 0; //add x 0
    x = eql(x, w); //eql x w
    x = eql(x, 0); //eql x 0
    y *= 0; //mul y 0
    y += 25; //add y 25
    y *= x; //mul y x
    y += 1; //add y 1
    z *= y; //mul z y
    y *= 0; //mul y 0
    y += w; //add y w
    y += 15; //add y 15
    y *= x; //mul y x
    z += y; //add z y

    z
}

fn calc_z(input: &Id) -> i128 {
    let A = [1, 1, 1, 26, 1, 1, 1, 26, 26, 1, 26, 26, 26, 26];
    let B = [11, 10, 13, -10, 11, 11, 12, -11, -7, 13, -13, 0, -11, 0];
    let C = [1, 10, 2, 5, 6, 0, 16, 12, 15, 7, 6, 5, 6, 15];

    let mut z = [0; INPUT_DIGITS_NUM];
    for i in 0..INPUT_DIGITS_NUM {
        let prev_z = if i == 0 { 0 } else { z[i - 1] };
        let x = (prev_z % 26) + B[i] != input[i];
        z[i] = (prev_z / A[i]) * if x { 26 } else { 1 } + if x { input[i] + C[i] } else { 0 };
    }

    //println!("{:?}", z);

    z[13]
}

struct SearchState {
    digit_index: usize,
    prev_z: i128,
    true_x_num: i32,
    show_progress: bool,
}
fn search_id(state: SearchState) -> Option<Id> {
    if state.digit_index == INPUT_DIGITS_NUM {
        if state.prev_z == 0 {
            return Some(vec![]);
        } else {
            return None;
        }
    }

    if state.true_x_num > 7 {
        return None;
    }

    let A = [1, 1, 1, 26, 1, 1, 1, 26, 26, 1, 26, 26, 26, 26];
    let B = [11, 10, 13, -10, 11, 11, 12, -11, -7, 13, -13, 0, -11, 0];
    let C = [1, 10, 2, 5, 6, 0, 16, 12, 15, 7, 6, 5, 6, 15];
    for digit in 1..=9 {
        if state.show_progress {
            println!("searching... first digit: {}", digit);
        }

        let x = (state.prev_z % 26) + B[state.digit_index] != digit;
        let z = (state.prev_z / A[state.digit_index]) * if x { 26 } else { 1 }
            + if x { digit + C[state.digit_index] } else { 0 };

        let next_state = SearchState {
            digit_index: state.digit_index + 1,
            prev_z: z,
            true_x_num: state.true_x_num + if x { 1 } else { 0 },
            show_progress: false,
        };
        if let Some(id) = search_id(next_state) {
            let mut new_id = id.clone();
            new_id.insert(0, digit);
            return Some(new_id);
        };
    }

    None
}

fn main() {
    //let mut num = (9 as u128).pow(INPUT_DIGITS_NUM as u32) - 1;
    let id = search_id(SearchState {
        digit_index: 0,
        prev_z: 0,
        true_x_num: 0,
        show_progress: true,
    })
    .unwrap();

    println!(
        "found id: {:?}, ALU validation: {:?}",
        id,
        orig_calc_z(&id) == 0
    );

    println!(
        "ans {}",
        id.iter().map(|x| x.to_string()).collect::<String>(),
    );
}

fn int2id(num: u128) -> Id {
    let str = format!(
        "{}{}",
        "0".repeat(INPUT_DIGITS_NUM),
        radix_fmt::radix_9(num)
    );
    str[str.len() - INPUT_DIGITS_NUM..]
        .chars()
        .map(|x| x.to_string().parse::<i128>().unwrap() + 1)
        .collect::<Id>()
}

#[cfg(test)]
mod tests {
    use crate::{calc_z, int2id, orig_calc_z};

    #[test]
    fn test_calc_z() {
        let mut num = 0;
        for _ in 0..10000 {
            let input = int2id(num);
            assert_eq!(orig_calc_z(&input), calc_z(&input));

            println!("{:?}", input);

            num += (9 as u128).pow(5);
        }
    }
}
