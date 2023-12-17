fn hash(input: &str) -> usize {
    let mut hash: usize = 0;
    for c in input.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}
fn main() {
    let res: usize = std::fs::read_to_string("input")
        .expect("Input file not found")
        .split(',')
        .map(hash)
        .sum();

    println!("{}", res);
}
