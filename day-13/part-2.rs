fn transpose(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = vec![vec!['0'; input.len()]; input[0].len()];
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            output[j][i] = *c;
        }
    }
    output
}

fn count_diff(a: &Vec<char>, b: &Vec<char>) -> (usize, usize) {
    let mut count = 0;
    let mut pos = 0;
    for ((i, &a), &b) in a.iter().enumerate().zip(b.iter()) {
        if a != b {
            pos = i;
            count += 1;
            if count >= 2 {
                return (count, pos);
            }
        }
    }
    (count, pos)
}
fn reflection_row(input: &Vec<Vec<char>>) -> Option<usize> {
    for idx in 0..input.len() - 1 {
        let mut up = idx;
        let mut down = up + 1;
        let mut reflects = true;
        let mut smudged = false;
        loop {
            if input[up] != input[down] {
                if smudged {
                    reflects = false;
                    break;
                }
                let (count, _) = count_diff(&input[up], &input[down]);
                if count == 1 {
                    smudged = true;
                } else {
                    reflects = false;
                }
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

        if reflects && smudged {
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
