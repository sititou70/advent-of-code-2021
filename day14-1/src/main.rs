use std::{
    collections::HashMap,
    io::{self, BufRead},
};

type Atom = char;

type Polymer = Vec<Atom>;

type PolymerTemplate = HashMap<(Atom, Atom), Atom>;

const STEPS: usize = 10;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut polymer = Polymer::new();
    for atom in lines.next().unwrap().unwrap().chars() {
        polymer.push(atom);
    }

    lines.next();

    let mut template = PolymerTemplate::new();
    for _line in lines {
        let line = _line.unwrap();
        let mut splited = line.split(" -> ");
        let input = splited.next().unwrap().chars().collect::<Vec<Atom>>();
        let output = splited.next().unwrap().chars().next().unwrap();
        template.insert((input[0], input[1]), output);
    }

    // grows
    for _ in 0..STEPS {
        polymer = grows_polymer(&polymer, &template);
    }

    // count
    let mut counts: HashMap<Atom, u32> = HashMap::new();
    for char in polymer {
        if !counts.contains_key(&char) {
            counts.insert(char, 0);
        }

        let count = counts.get(&char).unwrap();
        counts.insert(char, count + 1);
    }

    let mut max_count = u32::MIN;
    let mut min_count = u32::MAX;
    for (_, count) in counts {
        if max_count < count {
            max_count = count;
        }
        if min_count > count {
            min_count = count;
        }
    }

    println!("ans {}", max_count - min_count);
}

fn grows_polymer(polymer: &Polymer, template: &PolymerTemplate) -> Polymer {
    let mut new_polymer = Polymer::new();
    for i in 0..polymer.len() - 1 {
        let a1 = polymer.get(i).unwrap();
        let a2 = polymer.get(i + 1).unwrap();
        let gen_atom = template.get(&(*a1, *a2)).unwrap();

        new_polymer.push(*a1);
        new_polymer.push(*gen_atom);
    }
    new_polymer.push(*polymer.last().unwrap());

    new_polymer
}
