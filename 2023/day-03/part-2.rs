use core::num;
use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

fn read_file_into_matrix(file_path: &str) -> Result<Vec<Vec<char>>, Error> {
    let ret: Vec<Vec<char>> = std::fs::read_to_string(file_path)
        .expect("File does not exist")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Ok(ret)
}

fn find_stars_around(
    i: i32,
    j_start: i32,
    j_end: i32,
    matrix: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    let neighbours: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut ret = HashSet::new();
    let mut j = j_start;
    while j < j_end {
        for pos in neighbours.iter() {
            if i + pos.0 >= matrix.len() as i32 || i + pos.0 < 0 {
                continue;
            }

            if j + pos.1 >= matrix[0].len() as i32 || j + pos.1 < 0 {
                continue;
            }
            let x = (i + pos.0) as usize;
            let y = (j + pos.1) as usize;
            if matrix[x][y] == '*' {
                ret.insert((x, y));
            }
        }
        j += 1;
    }
    ret
}

fn main() {
    let mut hash: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let matrix = read_file_into_matrix("input").expect("");
    let mut i = 0;
    while i < matrix.len() {
        let mut j = 0;
        while j < matrix[0].len() {
            if !matrix[i][j].is_ascii_digit() {
                j += 1;
                continue;
            }
            let mut number = String::new();
            let start = j;
            let mut it = j as i32;
            while it >= 0 && matrix[i][it as usize].is_ascii_digit() {
                number.push(matrix[i][it as usize]);
                it -= 1;
            }
            number = number.chars().rev().collect();
            j = start + 1;
            while j < matrix[0].len() && matrix[i][j].is_ascii_digit() {
                number.push(matrix[i][j]);
                j += 1;
            }
            for (x, y) in find_stars_around(i as i32, it + 1, j as i32, &matrix).iter() {
                if let Some(l) = hash.get(&(*x, *y)) {
                    if let Ok(n) = number.parse::<u32>() {
                        let mut new = l.clone();
                        new.push(n);
                        hash.insert((*x, *y), new);
                    }
                } else {
                    let mut new = vec![];
                    if let Ok(n) = number.parse::<u32>() {
                        new.push(n);
                        hash.insert((*x, *y), new);
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
    let mut sum = 0;
    for (_, v) in hash.iter().filter(|(_, v)| v.len() == 2) {
        sum += v.iter().product::<u32>();
    }
    println!("{}", sum);
}
