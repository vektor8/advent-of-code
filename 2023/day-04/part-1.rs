fn main() {
    let res: u32 = std::fs::read_to_string("input")
        .expect("Input file does not exits")
        .lines()
        .map(|line| line.split(':').skip(1).collect::<Vec<&str>>()[0])
        .map(|card| {
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
                if count == 0 {
                    0
                } else {
                    2_u32.pow((count - 1) as u32)
                }
            } else {
                0
            }
        })
        .sum();
    println!("{}", res);
}
