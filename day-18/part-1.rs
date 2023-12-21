use std::{collections::VecDeque, vec};

#[derive(Debug)]
struct DigInstruction {
    dir: char,
    length: usize,
    color: usize,
}

fn find_inside_point(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for j in 0..grid[0].len() - 1 {
        for i in 0..grid.len() {
            if grid[i][j] == '#' && grid[i][j + 1] == '.' {
                return Some((i, j + 1));
            }
        }
    }
    None
}
fn flood_fill(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = grid.clone();

    let start = find_inside_point(&grid).unwrap();
    let mut q = VecDeque::new();
    q.push_back(start);
    grid[start.0][start.1] = '#';
    static DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    while let Some(pos) = q.pop_front() {
        for direction in DIRECTIONS {
            let next_pos = (pos.0 as i32 + direction.0, pos.1 as i32 + direction.1);
            if next_pos.0 >= 0
                && next_pos.0 < grid.len() as i32
                && next_pos.1 >= 0
                && next_pos.1 < grid[0].len() as i32
                && grid[next_pos.0 as usize][next_pos.1 as usize] == '.'
            {
                grid[next_pos.0 as usize][next_pos.1 as usize] = '#';
                q.push_back((next_pos.0 as usize, next_pos.1 as usize));
            }
        }
    }
    grid
}
fn main() {
    let dig_instructions: Vec<DigInstruction> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| {
            let mut line = line.split(' ');
            let dir = line.next().unwrap().chars().next().unwrap();
            let length = line.next().unwrap().parse::<usize>().unwrap();
            let color = usize::from_str_radix(&line.next().unwrap()[2..8], 16).unwrap();

            DigInstruction { dir, length, color }
        })
        .collect();

    let mut current: (i32, i32) = (0, 0);
    let mut all_pos: Vec<(i32, i32)> = vec![];
    for instr in dig_instructions {
        match instr.dir {
            'R' => {
                for _ in 0..instr.length {
                    all_pos.push(current);
                    current.1 += 1;
                }
            }
            'U' => {
                for _ in 0..instr.length {
                    all_pos.push(current);
                    current.0 -= 1;
                }
            }
            'D' => {
                for _ in 0..instr.length {
                    all_pos.push(current);
                    current.0 += 1;
                }
            }
            'L' => {
                for _ in 0..instr.length {
                    all_pos.push(current);
                    current.1 -= 1;
                }
            }
            _ => panic!("Invalid instruction"),
        }
    }

    let min_row = all_pos.iter().map(|t| t.0).min().unwrap();
    let min_col = all_pos.iter().map(|t| t.1).min().unwrap();

    let all_pos: Vec<(usize, usize)> = all_pos
        .iter()
        .map(|t| (t.0 - min_row, t.1 - min_col))
        .map(|t| (t.0 as usize, t.1 as usize))
        .collect();

    let rows = all_pos.iter().map(|t| t.0).max().unwrap() + 1;
    let cols = all_pos.iter().map(|t| t.1).max().unwrap() + 1;

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];

    for pos in all_pos {
        grid[pos.0][pos.1] = '#';
    }

    let grid = flood_fill(grid);
    let res = &grid.iter().flatten().filter(|&c| *c == '#').count();
    println!("{}", res);
}
