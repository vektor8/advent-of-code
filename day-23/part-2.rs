use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    current: (i32, i32),
    previous: (i32, i32),
    length: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.length.cmp(&self.length)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dfs(
    current: (usize, usize),
    goal: (usize, usize),
    graph: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    visited: &mut HashSet<(usize, usize)>,
    len: usize,
    max_len: &mut usize,
) {
    visited.insert(current);
    if current == goal {
        *max_len = len.max(*max_len);
    }

    for (&neighbour, distance) in graph.get(&current).unwrap() {
        {
            if !visited.contains(&neighbour) {
                dfs(neighbour, goal, graph, visited, len + distance, max_len);
            }
        }
    }
    visited.remove(&current);
}

fn edge_contraction(
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<char>>,
) -> HashMap<(usize, usize), HashMap<(usize, usize), usize>> {
    let mut points = vec![start, end];

    for (i, l) in grid.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            if c == '#' {
                continue;
            }
            let mut neighbour_count = 0;
            for direction in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let next_pos = (i as i32 + direction.0, j as i32 + direction.1);
                if next_pos.0 >= 0
                    && next_pos.0 < grid.len() as i32
                    && next_pos.1 >= 0
                    && next_pos.1 < grid[0].len() as i32
                    && grid[next_pos.0 as usize][next_pos.1 as usize] != '#'
                {
                    neighbour_count += 1;
                }
            }
            if neighbour_count >= 3 {
                points.push((i, j));
            }
        }
    }
    let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), usize>> =
        points.iter().map(|p| (*p, HashMap::new())).collect();

    for point in &points {
        let (si, sj) = *point;
        let mut stack = VecDeque::from([(0, si, sj)]);
        let mut visited = HashSet::from([(si, sj)]);

        while let Some((len, i, j)) = stack.pop_front() {
            if len != 0 && points.contains(&(i, j)) {
                let aux = graph.get_mut(point).unwrap();
                aux.insert((i, j), len);
                continue;
            }
            for direction in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let next_pos = (i as i32 + direction.0, j as i32 + direction.1);
                if next_pos.0 >= 0
                    && next_pos.0 < grid.len() as i32
                    && next_pos.1 >= 0
                    && next_pos.1 < grid[0].len() as i32
                    && grid[next_pos.0 as usize][next_pos.1 as usize] != '#'
                    && !visited.contains(&(next_pos.0 as usize, next_pos.1 as usize))
                {
                    stack.push_front((len + 1, next_pos.0 as usize, next_pos.1 as usize));
                    visited.insert((next_pos.0 as usize, next_pos.1 as usize));
                }
            }
        }
    }
    graph
}
fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start = (0, 1);
    let end = (input.len() - 1, input[0].len() - 2);
    let graph = edge_contraction((0, 1), (input.len() - 1, input[0].len() - 2), input);

    let mut res: usize = 0;
    dfs(start, end, &graph, &mut HashSet::new(), 0, &mut res);
    println!("{}", res);
}
