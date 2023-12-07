fn main() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(String::from)
        .collect();
    let time: Vec<&str> = input[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect();
    let time = time.join("").parse::<u64>().unwrap();

    let distance: Vec<&str> = input[1]
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect();
    let distance = distance.join("").parse::<u64>().unwrap();

    let first = (1..time / 2).find(|v| v * (time - v) > distance).unwrap();
    let res = time - first - first + 1;

    println!("{}", res);
}
