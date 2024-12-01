use std::collections::HashSet;

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
    current: (i32, i32),
    goal: (i32, i32),
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    len: u32,
    max_len: &mut u32,
) {
    visited.insert(current);
    if current == goal {
        *max_len = len.max(*max_len);
    }
    let directions = match grid[current.0 as usize][current.1 as usize] {
        '>' => vec![(0, 1)],
        'v' => vec![(1, 0)],
        '<' => vec![(0, -1)],
        '^' => vec![(-1, 0)],
        _ => vec![(0, 1), (1, 0), (-1, 0), (0, -1)],
    };

    for direction in directions {
        let next_pos = (current.0 + direction.0, current.1 + direction.1);
        if next_pos.0 >= 0
            && next_pos.0 < grid.len() as i32
            && next_pos.1 >= 0
            && next_pos.1 < grid[0].len() as i32
            && grid[next_pos.0 as usize][next_pos.1 as usize] != '#'
            && !visited.contains(&next_pos)
        {
            dfs(next_pos, goal, grid, visited, len + 1, max_len);
        }
    }
    visited.remove(&current);
}
fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut res: u32 = 0;
    dfs(
        (0, 1),
        (input.len() as i32 - 1, input[0].len() as i32 - 2),
        &input,
        &mut HashSet::new(),
        0,
        &mut res,
    );
    println!("{}", res);
}
