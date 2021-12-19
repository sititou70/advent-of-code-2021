use std::io::{self, BufRead};

use anyhow::{Context, Result};

#[derive(Debug)]
enum Node {
    Literal {
        version: u8,
        type_id: u8,
        value: u128,
    },
    Operator {
        version: u8,
        type_id: u8,
        nodes: Vec<Node>,
    },
}

type Data = Vec<char>;

// (Node, consumed)
fn parse(data: &Data) -> Result<(Node, u32)> {
    let type_id = u8::from_str_radix(&data[3..=5].iter().collect::<String>(), 2)
        .context(format!("invalid version, data: {:?}", data))?;

    match type_id {
        4 => parse_literal(&data),
        _ => parse_operator(&data),
    }
}

fn parse_literal(data: &Data) -> Result<(Node, u32)> {
    let version = u8::from_str_radix(&data[0..=2].iter().collect::<String>(), 2)
        .context(format!("invalid header, data: {:?}", data))?;

    let mut value_data: Vec<char> = Vec::new();
    let mut data_i = 6;
    loop {
        for j in data_i + 1..data_i + 5 {
            value_data.push(data[j]);
        }

        if data[data_i] == '0' {
            break;
        }
        data_i += 5;
    }

    let value = u128::from_str_radix(&value_data.iter().collect::<String>(), 2)
        .context(format!("invalid value, value_data: {:?}", value_data))?;

    Ok((
        Node::Literal {
            version,
            type_id: 4,
            value,
        },
        data_i as u32 + 5,
    ))
}

fn parse_operator(data: &Data) -> Result<(Node, u32)> {
    let mut consumed: u32 = 0;

    // header
    let version = u8::from_str_radix(&data[0..=2].iter().collect::<String>(), 2)
        .context(format!("invalid header, data: {:?}", data))?;
    let type_id = u8::from_str_radix(&data[3..=5].iter().collect::<String>(), 2)
        .context(format!("invalid version, data: {:?}", data))?;
    consumed += 6;

    // length_type_id: 0(sub packets bits, 15bit length), 1(packets num, 11bit length)
    let length_type_id = data[consumed as usize];
    consumed += 1;

    // sub-packets length
    let length_bits = if length_type_id == '0' { 15 } else { 11 };
    let sub_packet_length = u32::from_str_radix(
        &data[consumed as usize..(consumed + length_bits) as usize]
            .iter()
            .collect::<String>(),
        2,
    )
    .context(format!("invalid length format, data: {:?}", data))?;
    consumed += length_bits;

    // sub-packets
    let mut nodes: Vec<Node> = Vec::new();
    let mut sub_packet_consumed: u32 = 0;
    loop {
        let (node, consumed) = parse(&data[(consumed + sub_packet_consumed) as usize..].to_vec())?;
        nodes.push(node);
        sub_packet_consumed += consumed;

        if length_type_id == '0' {
            if sub_packet_consumed >= sub_packet_length {
                break;
            }
        } else {
            if nodes.len() >= sub_packet_length as usize {
                break;
            }
        }
    }
    consumed += sub_packet_consumed;

    Ok((
        Node::Operator {
            version,
            type_id,
            nodes,
        },
        consumed,
    ))
}

fn eval(node: &Node) -> Result<u128> {
    match node {
        Node::Literal {
            version: _,
            type_id: _,
            value,
        } => Ok(*value as u128),
        Node::Operator {
            version: _,
            type_id,
            nodes,
        } => {
            let reduce = |f: fn(x: u128, y: u128) -> u128| -> u128 {
                nodes
                    .iter()
                    .map(|node| eval(node).unwrap())
                    .reduce(f)
                    .unwrap()
            };
            let reduce_bool = |f: fn(x: u128, y: u128) -> bool| -> u128 {
                let n1 = eval(&nodes[0]).unwrap();
                let n2 = eval(&nodes[1]).unwrap();
                if f(n1, n2) {
                    1
                } else {
                    0
                }
            };
            match type_id {
                0 => Ok(reduce(|x, y| x + y)),
                1 => Ok(reduce(|x, y| x * y)),
                2 => Ok(reduce(|x, y| if x < y { x } else { y })),
                3 => Ok(reduce(|x, y| if x > y { x } else { y })),
                5 => Ok(reduce_bool(|x, y| x > y)),
                6 => Ok(reduce_bool(|x, y| x < y)),
                7 => Ok(reduce_bool(|x, y| x == y)),
                _ => Err(anyhow::anyhow!(format!("unknown type_id: {}", type_id))),
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut data = Data::new();
    for char in line.chars() {
        let num = u8::from_str_radix(&char.to_string(), 16).unwrap();
        for i in 0..=3 {
            if 1 << (3 - i) & num != 0 {
                data.push('1');
            } else {
                data.push('0');
            }
        }
    }

    let (node, _) = parse(&data).unwrap();
    println!("ans {}", eval(&node).unwrap());
}
