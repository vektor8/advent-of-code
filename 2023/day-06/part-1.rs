fn main() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(String::from)
        .collect();
    let times: Vec<u32> = input[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = input[1]
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let res: u32 = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            let first = (1..time / 2).find(|v| v * (time - v) > distance).unwrap();
            time - first - first + 1
        })
        .product();

    println!("{}", res);
}
