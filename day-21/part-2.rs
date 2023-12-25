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

fn reachable(start: (usize, usize), grid: &Grid, max_steps: usize) -> usize {
    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
    let mut q = VecDeque::from([(start, max_steps)]);

    visited[start.0][start.1] = 1;
    let mut res = 0;
    while let Some((next, steps)) = q.pop_front() {
        if steps % 2 == 0 {
            res += 1;
        }
        if steps == 0 {
            continue;
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
            q.push_back((new_pos, steps - 1));
        }
    }
    res
}

fn main() {
    let grid: Grid = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    assert!(grid.len() == grid[0].len());
    let size = grid.len();

    let steps = 26501365;
    let start = find_start(&grid);
    assert!(start.0 == start.1 && start.0 == size / 2);
    assert!(steps % size == size / 2);

    let grid_width = steps / size - 1;
    let odd_grids = (grid_width / 2 * 2 + 1).pow(2);
    let even_grids = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_grid_points = reachable(start, &grid, size * 2 + 1);
    let even_grid_points = reachable(start, &grid, size * 2);

    let corner_t = reachable((size - 1, start.1), &grid, size - 1);
    let corner_r = reachable((start.0, 0), &grid, size - 1);
    let corner_b = reachable((0, start.1), &grid, size - 1);
    let corner_l = reachable((start.0, size - 1), &grid, size - 1);

    let small_tr = reachable((size - 1, 0), &grid, size / 2 - 1);
    let small_tl = reachable((size - 1, size - 1), &grid, size / 2 - 1);
    let small_br = reachable((0, 0), &grid, size / 2 - 1);
    let small_bl = reachable((0, size - 1), &grid, size / 2 - 1);

    let large_tr = reachable((size - 1, 0), &grid, size * 3 / 2 - 1);
    let large_tl = reachable((size - 1, size - 1), &grid, size * 3 / 2 - 1);
    let large_br = reachable((0, 0), &grid, size * 3 / 2 - 1);
    let large_bl = reachable((0, size - 1), &grid, size * 3 / 2 - 1);

    let res = odd_grids * odd_grid_points
        + even_grids * even_grid_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
        + grid_width * (large_tr + large_tl + large_br + large_bl);
    println!("{}", res);
}
