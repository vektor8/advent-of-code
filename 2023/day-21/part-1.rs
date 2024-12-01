use std::collections::VecDeque;

type Grid = Vec<Vec<char>>;
fn find_start(grid: &Grid) -> (usize, usize) {
    for (i, l) in grid.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c == 'S' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn bfs(start: (usize, usize), grid: &Grid, max_steps: usize) -> Vec<Vec<usize>> {
    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
    let mut q = VecDeque::from([start]);

    visited[start.0][start.1] = 1;
    while let Some(next) = q.pop_front() {
        if visited[next.0][next.1] == max_steps {
            break;
        }
        for (x, y) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = (next.0 as i64 + x, next.1 as i64 + y);
            if new_pos.0 < 0
                || new_pos.0 >= grid.len() as i64
                || new_pos.1 < 0
                || new_pos.1 >= grid[0].len() as i64
            {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if grid[new_pos.0][new_pos.1] == '#' || visited[new_pos.0][new_pos.1] != 0 {
                continue;
            }
            visited[new_pos.0][new_pos.1] = visited[next.0][next.1] + 1;
            q.push_back(new_pos);
        }
    }
    visited
}

fn main() {
    let grid: Grid = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let res = bfs(find_start(&grid), &grid, 65)
        .iter()
        .flatten()
        .filter(|&v| *v != 0 && (*v - 1) % 2 == 0)
        .count();
    println!("{}", res);
}
