fn move_north(input: &mut Vec<Vec<char>>) {
    for i in 1..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] != 'O' || input[i - 1][j] != '.' {
                continue;
            }
            let mut k = i;

            loop {
                if input[k][j] == '.' && (k == 0 || input[k - 1][j] != '.') {
                    break;
                }
                k -= 1;
            }
            if k != i {
                input[k][j] = 'O';
                input[i][j] = '.';
            }
        }
    }
}

fn move_south(input: &mut Vec<Vec<char>>) {
    for i in (0..input.len() - 1).rev() {
        for j in 0..input[0].len() {
            if input[i][j] != 'O' || input[i + 1][j] != '.' {
                continue;
            }
            let mut k = i;

            loop {
                if input[k][j] == '.' && (k == input.len() - 1 || input[k + 1][j] != '.') {
                    break;
                }
                k += 1;
            }
            if k != i {
                input[k][j] = 'O';
                input[i][j] = '.';
            }
        }
    }
}

fn move_east(input: &mut Vec<Vec<char>>) {
    for j in (0..input[0].len() - 1).rev() {
        for i in 0..input.len() {
            if input[i][j] != 'O' || input[i][j + 1] != '.' {
                continue;
            }
            let mut k = j;

            loop {
                if input[i][k] == '.' && (k == input[0].len() - 1 || input[i][k + 1] != '.') {
                    break;
                }
                k += 1;
            }
            if k != j {
                input[i][k] = 'O';
                input[i][j] = '.';
            }
        }
    }
}

fn move_west(input: &mut Vec<Vec<char>>) {
    for j in 1..input[0].len() {
        for i in 0..input.len() {
            if input[i][j] != 'O' || input[i][j - 1] != '.' {
                continue;
            }
            let mut k = j;

            loop {
                if input[i][k] == '.' && (k == 0 || input[i][k - 1] != '.') {
                    break;
                }
                k -= 1;
            }
            if k != j {
                input[i][k] = 'O';
                input[i][j] = '.';
            }
        }
    }
}
fn main() {
    let mut input: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut states: Vec<Vec<Vec<char>>> = vec![];
    for i in 1..1000000001 {
        if i % 1_000_000 == 0 {
            println!("{}", i);
        }
        move_north(&mut input);
        move_west(&mut input);
        move_south(&mut input);
        move_east(&mut input);

        if states.contains(&input) {
            let cycle_start = states.iter().position(|e| e.clone() == input).unwrap();
            let cycle_end = i - 1;
            let cycle_len = cycle_end - cycle_start;
            let final_idx = (1_000_000_000 - i) % cycle_len;
            input = states[cycle_start + final_idx].clone();
            break;
        }
        states.push(input.clone());
    }

    let res: usize = input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .map(|&c| if c == 'O' { input.len() - i } else { 0 })
                .sum::<usize>()
        })
        .sum();

    println!("{}", res);
}
