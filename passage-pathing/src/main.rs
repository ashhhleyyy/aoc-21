use std::{collections::HashMap, io::BufRead, str::FromStr};

fn main() {
    let stdin = std::io::stdin();
    let mut graph = HashMap::<Node, Vec<Node>>::new();
    let mut start_node = None;
    for line in stdin.lock().lines().map(Result::unwrap) {
        let (a, b) = line.split_once("-").unwrap();
        let (a, b) = (a.parse::<Node>().unwrap(), b.parse::<Node>().unwrap());
        graph.entry(a.clone()).or_insert_with(Vec::new).push(b.clone());
        graph.entry(b.clone()).or_insert_with(Vec::new).push(a.clone());
        if a.name() == "start" {
            start_node = Some(a.clone());
        } else if b.name() == "start" {
            start_node = Some(b.clone());
        }
    }
    let start_node = start_node.unwrap();
    
    let mut paths = Vec::<Vec<Node>>::new();
    let current_path = vec![&start_node];
    find_paths(&mut paths, &graph, &current_path, start_node.clone());
    println!("{}", paths.len());
}

fn find_paths(paths: &mut Vec<Vec<Node>>, graph: &HashMap<Node, Vec<Node>>, current_path: &Vec<&Node>, start_node: Node) {
    let connections = graph.get(&start_node).unwrap();
    for node in connections {
        if node.is_smol() && current_path.contains(&node) {
            continue;
        }

        let mut new_path = current_path.clone();
        new_path.push(&node);
        if node.name() == "end" {
            // so cursed but oh well
            paths.push(new_path.iter().map(Clone::clone).map(Clone::clone).collect::<Vec<_>>());
        } else {
            find_paths(paths, graph, &new_path, node.clone());
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Node {
    Large(String),
    Small(String),
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.chars().next().unwrap().is_lowercase() {
            Self::Small(s.to_string())
        } else {
            Self::Large(s.to_string())
        })
    }
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::Large(name) => name,
            Node::Small(name) => name,
        }
    }

    fn is_smol(&self) -> bool {
        match self {
            Node::Large(_) => false,
            Node::Small(_) => true,
        }
    }
}
