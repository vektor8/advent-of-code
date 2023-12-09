use std::collections::VecDeque;

fn find_next(seq: Vec<i32>) -> i32 {
    let mut stack: VecDeque<Vec<i32>> = VecDeque::from(vec![seq]);
    let mut res: i32;
    loop {
        let mut next: Vec<i32> = vec![];
        let current = stack.front().unwrap();
        for pair in current.windows(2) {
            next.push(pair[1] - pair[0]);
        }
        res = *next.first().unwrap();
        if next.iter().min() == next.iter().max() {
            break;
        }
        stack.push_front(next);
    }
    while !stack.is_empty() {
        res = stack.pop_front().unwrap().first().unwrap() - res;
    }
    res
}
fn main() {
    let res: i32 = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(find_next)
        .sum();
    println!("{}", res);
}
