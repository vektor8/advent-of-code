use std::collections::VecDeque;

fn main() {
    let mut q: VecDeque<usize> = VecDeque::new();
    let values: Vec<usize> = std::fs::read_to_string("input")
        .expect("Input file does not exits")
        .lines()
        .map(|line| line.split(':').skip(1).collect::<Vec<&str>>()[0])
        .enumerate()
        .map(|(idx, card)| {
            let mut it = card.trim().split(" | ");
            let winning_numbers: Option<Vec<u32>> = it.next().map(|winning| {
                winning
                    .split(' ')
                    .filter(|number_str| !number_str.is_empty())
                    .map(|number_str| {
                        number_str
                            .parse::<u32>()
                            .expect("Not a number in the winning numbers")
                    })
                    .collect()
            });

            let my_numbers: Option<Vec<u32>> = it.next().map(|mine| {
                mine.split(' ')
                    .filter(|number_str| !number_str.is_empty())
                    .map(|number_str| {
                        number_str
                            .parse::<u32>()
                            .expect("Not a number in my numbers")
                    })
                    .collect()
            });

            if let (Some(my_numbers), Some(winning_numbers)) = (my_numbers, winning_numbers) {
                let count = my_numbers
                    .iter()
                    .filter(|nr| winning_numbers.contains(nr))
                    .count();
                for i in (idx + 2)..(idx + 2 + count) {
                    q.push_front(i);
                }
                count
            } else {
                0
            }
        })
        .collect();
    let mut res = values.len();
    while !q.is_empty() {
        res += 1;
        let e = q.pop_back().unwrap();
        for i in e + 1..e + values[e - 1] + 1 {
            q.push_front(i);
        }
    }
    println!("{}", res);
}
