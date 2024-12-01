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
fn main() {
    let grid: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut visited = HashMap::new();
    traverse((0, 0), Direction::Right, &grid, &mut visited);

    let mut answer = HashSet::new();
    for k in visited.keys() {
        answer.insert((k.0, k.1));
    }
    let res = answer.len();
    println!("{}", res);
}
