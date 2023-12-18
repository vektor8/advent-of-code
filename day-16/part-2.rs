use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn traverse(
    position: (i32, i32),
    direction: Direction,
    grid: &Vec<Vec<char>>,
    visited: &mut HashMap<(i32, i32, Direction), bool>,
) {
    if position.0 < 0
        || position.0 as usize >= grid.len()
        || position.1 < 0
        || position.1 as usize >= grid[0].len()
    {
        return;
    }
    let key = (position.0, position.1, direction);
    if let Some(&is_visited) = visited.get(&key) {
        if is_visited {
            return;
        }
    }

    visited.insert(key, true);
    match grid[position.0 as usize][position.1 as usize] {
        '.' => match direction {
            Direction::Up => {
                traverse((position.0 - 1, position.1), direction, grid, visited);
            }
            Direction::Down => {
                traverse((position.0 + 1, position.1), direction, grid, visited);
            }
            Direction::Left => {
                traverse((position.0, position.1 - 1), direction, grid, visited);
            }
            Direction::Right => {
                traverse((position.0, position.1 + 1), direction, grid, visited);
            }
        },
        '\\' => match direction {
            Direction::Up => traverse((position.0, position.1 - 1), Direction::Left, grid, visited),
            Direction::Down => traverse(
                (position.0, position.1 + 1),
                Direction::Right,
                grid,
                visited,
            ),
            Direction::Left => traverse((position.0 - 1, position.1), Direction::Up, grid, visited),
            Direction::Right => {
                traverse((position.0 + 1, position.1), Direction::Down, grid, visited)
            }
        },
        '/' => match direction {
            Direction::Up => traverse(
                (position.0, position.1 + 1),
                Direction::Right,
                grid,
                visited,
            ),
            Direction::Down => {
                traverse((position.0, position.1 - 1), Direction::Left, grid, visited)
            }
            Direction::Left => {
                traverse((position.0 + 1, position.1), Direction::Down, grid, visited)
            }
            Direction::Right => {
                traverse((position.0 - 1, position.1), Direction::Up, grid, visited)
            }
        },

        '-' => match direction {
            Direction::Up => {
                traverse((position.0, position.1 - 1), Direction::Left, grid, visited);
                traverse(
                    (position.0, position.1 + 1),
                    Direction::Right,
                    grid,
                    visited,
                )
            }
            Direction::Down => {
                traverse((position.0, position.1 - 1), Direction::Left, grid, visited);
                traverse(
                    (position.0, position.1 + 1),
                    Direction::Right,
                    grid,
                    visited,
                )
            }
            Direction::Left => {
                traverse((position.0, position.1 - 1), Direction::Left, grid, visited)
            }
            Direction::Right => traverse(
                (position.0, position.1 + 1),
                Direction::Right,
                grid,
                visited,
            ),
        },
        '|' => match direction {
            Direction::Up => {
                traverse((position.0 - 1, position.1), Direction::Up, grid, visited);
            }
            Direction::Down => {
                traverse((position.0 + 1, position.1), Direction::Down, grid, visited);
            }
            Direction::Left => {
                traverse((position.0 - 1, position.1), Direction::Up, grid, visited);
                traverse((position.0 + 1, position.1), Direction::Down, grid, visited)
            }
            Direction::Right => {
                traverse((position.0 - 1, position.1), Direction::Up, grid, visited);
                traverse((position.0 + 1, position.1), Direction::Down, grid, visited)
            }
        },
        _ => {}
    }
}

fn get_energised(start_pos: (i32, i32), direction: Direction, grid: &Vec<Vec<char>>) -> usize {
    let mut visited = HashMap::new();

    traverse(start_pos, direction, grid, &mut visited);
    let mut answer = HashSet::new();
    let mut sol_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
    for k in visited.keys() {
        answer.insert((k.0, k.1));
        sol_grid[k.0 as usize][k.1 as usize] = '#';
    }
    answer.len()
}
fn main() {
    let grid: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let upper_max = (0..grid[0].len())
        .map(|column| get_energised((0, column as i32), Direction::Down, &grid))
        .max();

    let lower_max = (0..grid[0].len())
        .map(|column| {
            get_energised(
                ((grid.len() - 1) as i32, column as i32),
                Direction::Up,
                &grid,
            )
        })
        .max();

    let left_max = (0..grid.len())
        .map(|row| get_energised((0, row as i32), Direction::Right, &grid))
        .max();

    let right_max = (0..grid.len())
        .map(|row| {
            get_energised(
                ((grid[0].len() - 1) as i32, row as i32),
                Direction::Right,
                &grid,
            )
        })
        .max();

    let res = upper_max
        .max(lower_max)
        .max(left_max)
        .max(right_max)
        .unwrap();

    println!("{}", res);
}
