use std::vec;

#[derive(Debug)]
struct DigInstruction {
    dir: char,
    length: usize,
    color: usize,
}

// fn find_inside_point(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
//     for j in 0..grid[0].len() - 1 {
//         for i in 0..grid.len() {
//             if grid[i][j] == '#' && grid[i][j + 1] == '.' {
//                 return Some((i, j + 1));
//             }
//         }
//     }
//     None
// }
// fn flood_fill(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
//     let mut grid = grid.clone();

//     let start = find_inside_point(&grid).unwrap();
//     let mut q = VecDeque::new();
//     q.push_back(start);
//     grid[start.0][start.1] = '#';
//     static DIRECTIONS: [(i64, i64); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
//     while let Some(pos) = q.pop_front() {
//         for direction in DIRECTIONS {
//             let next_pos = (pos.0 as i64 + direction.0, pos.1 as i64 + direction.1);
//             if next_pos.0 >= 0
//                 && next_pos.0 < grid.len() as i64
//                 && next_pos.1 >= 0
//                 && next_pos.1 < grid[0].len() as i64
//                 && grid[next_pos.0 as usize][next_pos.1 as usize] == '.'
//             {
//                 grid[next_pos.0 as usize][next_pos.1 as usize] = '#';
//                 q.push_back((next_pos.0 as usize, next_pos.1 as usize));
//             }
//         }
//     }
//     grid
// }

#[derive(Debug)]
struct Range {
    start: (i64, i64),
    end: (i64, i64),
    dir: char,
}

impl Range {
    fn contains(&self, x: (i64, i64)) -> bool {
        match self.dir {
            'U' => {
                if x.1 != self.end.1 {
                    false
                } else {
                    x.0 > self.end.0 && x.0 < self.start.0
                }
            }
            'D' => {
                if x.1 != self.end.1 {
                    false
                } else {
                    x.0 < self.end.0 && x.0 > self.start.0
                }
            }
            'R' => {
                if x.0 != self.end.0 {
                    false
                } else {
                    x.1 < self.end.1 && x.0 > self.start.1
                }
            }
            'L' => {
                if x.0 != self.end.0 {
                    false
                } else {
                    x.1 > self.end.1 && x.0 < self.start.1
                }
            }
            _ => {
                panic!("Unknown direction")
            }
        }
    }
}
fn main() {
    let dig_instructions: Vec<DigInstruction> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| {
            let mut line = line.split(' ');
            line.next();
            line.next();
            let color = line.next().unwrap();
            let dir = match color.chars().nth(7).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("Unknown direction"),
            };
            let length = usize::from_str_radix(&color[2..7], 16).unwrap();

            DigInstruction {
                dir,
                length,
                color: 0,
            }
        })
        .collect();

    let mut current: (i64, i64) = (0, 0);
    let mut points: Vec<(i64, i64)> = vec![current];
    let mut total_boundary: i64 = 0;
    for instr in dig_instructions {
        total_boundary += instr.length as i64;
        match instr.dir {
            'R' => {
                current = (current.0, current.1 + instr.length as i64);
                points.push(current);
            }
            'U' => {
                current = (current.0 - instr.length as i64, current.1);
                points.push(current);
            }
            'D' => {
                current = (current.0 + instr.length as i64, current.1);
                points.push(current);
            }
            'L' => {
                current = (current.0, current.1 - instr.length as i64);
                points.push(current);
            }
            _ => panic!("Invalid instruction"),
        }
    }

    println!("{:?}", points);

    let mut area = points[0].0 * (points[1].1 - points[points.len() - 1].1)
        + points[points.len() - 1].0 * (points[0].1 - points[points.len() - 2].1);

    for i in 1..points.len() - 1 {
        area += points[i].0 * (points[i + 1].1 - points[i - 1].1);
    }
    area = area.abs() / 2;
    let i = area - total_boundary / 2 + 1;
    let res = i + total_boundary;
    println!("{}", res);
}
