use std::{
    collections::HashMap,
    io::{self, BufRead},
};

type Atom = char;

type AtomCount = HashMap<Atom, u128>;

type Molecule = (Atom, Atom);

type MoleculesCount = HashMap<Molecule, u128>;

type PolymerTemplate = HashMap<(Atom, Atom), Atom>;

const STEPS: usize = 40;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut atom_count = AtomCount::new();
    let mut molecules_count = MoleculesCount::new();

    let initial_polymer = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<Atom>>();

    for i in 0..initial_polymer.len() - 1 {
        let a1 = initial_polymer.get(i).unwrap();
        let a2 = initial_polymer.get(i + 1).unwrap();
        *molecules_count.entry((*a1, *a2)).or_insert(0) += 1;
        *atom_count.entry(*a1).or_insert(0) += 1;
    }
    *atom_count
        .entry(*initial_polymer.last().unwrap())
        .or_insert(0) += 1;

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
        let mut new_molecules_count = MoleculesCount::new();
        for (molecule, count) in &molecules_count {
            let generated_atom = template.get(&molecule).unwrap();

            *new_molecules_count
                .entry((molecule.0, *generated_atom))
                .or_insert(0) += count;
            *new_molecules_count
                .entry((*generated_atom, molecule.1))
                .or_insert(0) += count;

            *atom_count.entry(*generated_atom).or_insert(0) += count;
        }

        molecules_count = new_molecules_count;
    }

    let mut max_count = u128::MIN;
    let mut min_count = u128::MAX;
    for (_, count) in atom_count {
        if max_count < count {
            max_count = count;
        }
        if min_count > count {
            min_count = count;
        }
    }

    println!("ans {}", max_count - min_count);
}
