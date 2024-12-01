use rand::seq::IteratorRandom;
use std::collections::{HashMap, HashSet, VecDeque};

type Node = [char; 3];
type Edge = (Node, Node);
type Graph = HashMap<Node, Vec<Node>>;

fn furthest(graph: &Graph, start: Node) -> Node {
    let mut q = VecDeque::from([start]);
    let mut visited = HashSet::new();
    let mut result = start;

    while let Some(current) = q.pop_front() {
        result = current;
        visited.insert(current);
        for n in graph.get(&current).unwrap() {
            if visited.contains(n) {
                continue;
            }
            q.push_back(*n);
        }
    }
    result
}

fn max_flow(graph: &Graph, start: Node, end: Node) -> usize {
    let mut used_edges: Vec<Edge> = vec![];
    let mut mst_size: usize = 0;

    for _ in 0..4 {
        let mut q = VecDeque::from([start]);
        let mut visited = HashSet::from([start]);
        let mut parents: HashMap<Node, Node> = HashMap::new();
        mst_size = 0;
        while let Some(current) = q.pop_front() {
            mst_size += 1;
            if current == end {
                let mut node = end;
                while let Some(&next) = parents.get(&node) {
                    used_edges.push((node, next));
                    used_edges.push((next, node));
                    node = next;
                }
                break;
            }
            for &n in graph.get(&current).unwrap() {
                if visited.contains(&n) || used_edges.contains(&(current, n)) {
                    continue;
                }
                parents.insert(n, current);
                visited.insert(n);
                q.push_back(n);
            }
        }
    }
    mst_size
}

fn solve(graph: &Graph) -> usize {
    let dummy = graph.keys().choose(&mut rand::thread_rng()).unwrap();

    let start = furthest(graph, *dummy);
    let end = furthest(graph, start);

    let mst_size = max_flow(graph, start, end);
    (graph.keys().count() - mst_size) * mst_size
}
fn main() {
    let mut graph = Graph::new();

    for line in std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
    {
        let start: Vec<char> = line[..3].chars().collect();
        let start: Node = [start[0], start[1], start[2]];

        let neighbours_list: Vec<Node> = line[5..]
            .split(' ')
            .map(|neighbour| {
                let n: Vec<char> = neighbour[..3].chars().collect();
                [n[0], n[1], n[2]]
            })
            .collect();

        match graph.get_mut(&start) {
            Some(l) => l.extend(neighbours_list.iter()),
            None => {
                graph.insert(start, neighbours_list.clone());
            }
        }

        for n in neighbours_list {
            match graph.get_mut(&n) {
                Some(l) => l.push(start),
                None => {
                    graph.insert(n, vec![start]);
                }
            }
        }
    }

    println!("{}", solve(&graph));
}
