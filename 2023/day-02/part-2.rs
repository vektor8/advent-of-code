fn main() {
    let mut sum = 0;
    for line in std::fs::read_to_string("input")
        .expect("File input does not exist")
        .lines()
    {
        let mut split = line.split(": ");
        let (_, data) = (split.next().unwrap(), split.next().unwrap());
        // game_id = game_id.split(' ').nth(1).unwrap();
        // let id: u32 = game_id.parse().unwrap();
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        for round in data.split("; ") {
            for ball_round in round.split(", ") {
                let mut iter = ball_round.split(' ').take(2);
                let count = iter.next().unwrap().parse::<u32>().unwrap();
                match iter.next().unwrap() {
                    "red" => {
                        if count > max_red {
                            max_red = count;
                        }
                    }
                    "green" => {
                        if count > max_green {
                            max_green = count;
                        }
                    }
                    "blue" => {
                        if count > max_blue {
                            max_blue = count;
                        }
                    }
                    _ => {}
                }
            }
        }

        sum += max_red * max_green * max_blue;
    }
    println!("{}", sum)
}
