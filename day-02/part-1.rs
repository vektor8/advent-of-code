fn main() {
    let (red, green, blue) = (12, 13, 14);
    let mut sum = 0;
    for line in std::fs::read_to_string("input").unwrap().lines() {
        let mut split = line.split(": ");
        let (mut game_id, data) = (split.next().unwrap(), split.next().unwrap());
        game_id = game_id.split(' ').nth(1).unwrap();
        let id: u32 = game_id.parse().unwrap();
        let mut flag = true;
        for round in data.split("; ") {
            for ball_round in round.split(", ") {
                let mut iter = ball_round.split(' ').take(2);
                let count = iter.next().unwrap().parse::<u32>().unwrap();
                match iter.next().unwrap() {
                    "red" => {
                        if count > red {
                            flag = false;
                            break;
                        }
                    }
                    "green" => {
                        if count > green {
                            flag = false;
                            break;
                        }
                    }
                    "blue" => {
                        if count > blue {
                            flag = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        if flag {
            sum += id;
        }
    }
    println!("{}", sum)
}
