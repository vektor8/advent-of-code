fn main() {
    let mut input: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
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
