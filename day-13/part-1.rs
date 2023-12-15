fn transpose(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = vec![vec!['0'; input.len()]; input[0].len()];
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            output[j][i] = *c;
        }
    }
    output
}

fn reflection_row(input: &Vec<Vec<char>>) -> Option<usize> {
    for idx in 0..input.len() - 1 {
        let mut up = idx;
        let mut down = up + 1;
        let mut reflects = true;
        loop {
            if input[up] != input[down] {
                reflects = false;
            }
            if up == 0 {
                break;
            }
            if down == input.len() - 1 {
                break;
            }
            up -= 1;
            down += 1;
        }

        if reflects {
            return Some(idx);
        }
    }
    None
}

fn main() {
    let patterns: Vec<Vec<Vec<char>>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let res: usize = patterns
        .iter()
        .map(|pat| {
            if let Some(idx) = reflection_row(pat) {
                return 100 * (idx + 1);
            } else {
                if let Some(idx) = reflection_row(&transpose(pat)) {
                    return idx + 1;
                } else {
                    panic!("No pattern found");
                }
            }
        })
        .sum();

    println!("{}", res);
}
