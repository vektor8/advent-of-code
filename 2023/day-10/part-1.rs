use std::{collections::VecDeque, vec};

fn find_start(input: &[Vec<char>]) -> Option<(usize, usize)> {
    for (x, l) in input.iter().enumerate() {
        for (y, &c) in l.iter().enumerate() {
            if c == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn get_reachable_neighbours(pos: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let x_size = grid.len() as i32;
    let y_size = grid[0].len() as i32;
    let pos = (pos.0 as i32, pos.1 as i32);
    match grid[pos.0 as usize][pos.1 as usize] {
        '|' => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
        '-' => vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
        'L' => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)],
        'J' => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)],
        '7' => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)],
        'F' => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
        'S' => vec![
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ],
        _ => vec![],
    }
    .iter()
    .filter(|p| p.0 >= 0 && p.0 < x_size && p.1 >= 0 && p.1 < y_size)
    .map(|e: &(i32, i32)| (e.0 as usize, e.1 as usize))
    .collect()
}

fn find_longest_loop(start_pos: (usize, usize), grid: &Vec<Vec<char>>) -> usize {
    let mut step_grid: Vec<Vec<usize>> = vec![0; grid.len()]
        .iter()
        .map(|_| vec![0; grid[0].len()])
        .collect();
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::from([(start_pos, 1)]);
    let mut possible_lens: Vec<usize> = vec![];
    while !queue.is_empty() {
        let (pos, len) = queue.pop_front().unwrap();
        step_grid[pos.0][pos.1] = len;
        get_reachable_neighbours(pos, grid).iter().for_each(|&n| {
            if step_grid[n.0][n.1] == 0 {
                queue.push_front((n, len + 1));
            } else if n == start_pos {
                possible_lens.push(len);
            }
        });
    }
    possible_lens.iter().max().unwrap().to_owned()
}

fn solve(input: &Vec<Vec<char>>) {
    let start_pos = find_start(input);
    if start_pos.is_none() {
        panic!("No start position in grid");
    }
    let start_pos = start_pos.unwrap();
    let len = find_longest_loop(start_pos, input);
    println!("{}", len / 2);
}

fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    solve(&input);
}
