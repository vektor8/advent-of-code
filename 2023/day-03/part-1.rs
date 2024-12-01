use std::io::Error;

fn read_file_into_matrix(file_path: &str) -> Result<Vec<Vec<char>>, Error> {
    let ret: Vec<Vec<char>> = std::fs::read_to_string(file_path)
        .expect("File does not exist")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Ok(ret)
}

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

fn has_symbol_arround(i: i32, j: i32, matrix: &Vec<Vec<char>>) -> bool {
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

    for pos in neighbours.iter() {
        if i + pos.0 >= matrix.len() as i32 || i + pos.0 < 0 {
            continue;
        }

        if j + pos.1 >= matrix[0].len() as i32 || j + pos.1 < 0 {
            continue;
        }
        if is_symbol(matrix[(i + pos.0) as usize][(j + pos.1) as usize]) {
            return true;
        }
    }
    false
}
fn main() {
    let mut sum = 0;
    let matrix = read_file_into_matrix("input").expect("");
    let mut i = 0;
    while i < matrix.len() {
        let mut j = 0;
        while j < matrix[0].len() {
            if !(matrix[i][j].is_ascii_digit() && has_symbol_arround(i as i32, j as i32, &matrix)) {
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
            if let Ok(n) = number.parse::<u32>() {
                sum += n;
            }
            j += 1;
        }
        i += 1;
    }
    println!("{}", sum);
}
