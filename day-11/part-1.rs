fn get_expandable(input: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let rows_without_galaxies: Vec<usize> = input
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect();

    let mut columns_without_galaxies = vec![];

    for j in 0..input[0].len() {
        let mut no_galaxy = true;
        for i in 0..input.len() {
            if input[i][j] != '.' {
                no_galaxy = false;
                break;
            }
        }
        if no_galaxy {
            columns_without_galaxies.push(j);
        }
    }
    (rows_without_galaxies, columns_without_galaxies)
}

fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let (expandable_rows, expandable_cols) = get_expandable(&input);

    let galaxies: Vec<(usize, usize)> = input
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, &c)| c == '#')
        .map(|(i, _)| (i / input[0].len(), i % input[0].len()))
        .collect();

    let mut res = 0;
    for i in 0..galaxies.len() - 1 {
        for j in (i + 1)..galaxies.len() {
            res += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
            if galaxies[i].0 > galaxies[j].0 {
                res += (galaxies[j].0..galaxies[i].0)
                    .filter(|i| expandable_rows.contains(i))
                    .count()
            } else {
                res += (galaxies[i].0..galaxies[j].0)
                    .filter(|i| expandable_rows.contains(i))
                    .count()
            }

            if galaxies[i].1 > galaxies[j].1 {
                res += (galaxies[j].1..galaxies[i].1)
                    .filter(|i| expandable_cols.contains(i))
                    .count()
            } else {
                res += (galaxies[i].1..galaxies[j].1)
                    .filter(|i| expandable_cols.contains(i))
                    .count()
            }
        }
    }
    println!("{}", res);
}
