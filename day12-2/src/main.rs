use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

type Node = String;
type Path = Vec<Node>;

struct Map(HashMap<Node, HashSet<Node>>);
impl Map {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_single_edge(&mut self, n1: &Node, n2: &Node) {
        if !self.0.contains_key(n1) {
            self.0.insert(n1.clone(), HashSet::new());
        };

        let vec = self.0.get_mut(n1).unwrap();
        vec.insert(n2.clone());
    }
    pub fn add_edge(&mut self, n1: &Node, n2: &Node) {
        self.add_single_edge(n1, n2);
        self.add_single_edge(n2, n1);
    }
    pub fn get_next_node(&self, node: &Node) -> Vec<&Node> {
        let next_nodes = self.0.get(node);
        match next_nodes {
            Some(set) => set.iter().collect(),
            None => vec![],
        }
    }
}

fn visit(path: &Path, map: &Map) -> Vec<Path> {
    let mut paths: Vec<Path> = vec![];

    let current_node = path.last().unwrap();
    if current_node == "end" {
        paths.push(path.clone());
        return paths;
    }

    let lower_path = path
        .iter()
        .filter(|x| x.chars().next().unwrap().is_lowercase())
        .collect::<Vec<&String>>();
    let mut lower_nodes: HashSet<Node> = HashSet::new();
    for node in &lower_path {
        lower_nodes.insert(node.to_string());
    }
    let double_visited = lower_path.len() != lower_nodes.len();
    for next_node in map.get_next_node(current_node) {
        if next_node == "start" {
            continue;
        }
        if next_node.chars().next().unwrap().is_lowercase()
            && path
                .iter()
                .filter(|x| x == &next_node)
                .collect::<Vec<&Node>>()
                .len()
                >= if double_visited { 1 } else { 2 }
        {
            continue;
        }

        let mut next_path = path.clone();
        next_path.push(next_node.clone());
        let mut found_paths = visit(&next_path, map);
        paths.append(&mut found_paths);
    }

    paths
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut map = Map::new();
    for _line in lines {
        let line = _line.unwrap();
        let splited = line.split("-").collect::<Vec<&str>>();
        map.add_edge(&splited[0].to_owned(), &splited[1].to_owned());
    }

    let paths = visit(&vec!["start".to_owned()], &map);
    println!("ans {}", paths.len());
}
