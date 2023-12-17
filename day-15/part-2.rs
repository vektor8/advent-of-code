fn hash(input: &str) -> usize {
    let mut hash: usize = 0;
    for c in input.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}
#[derive(Clone)]
struct Lens<'a> {
    chars: &'a str,
    focal_length: usize,
}
fn main() {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    let input: Vec<String> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .split(',')
        .map(|s| s.to_owned())
        .collect();
    input.iter().for_each(|s| {
        if s.contains('=') {
            let mut s = s.split('=');
            let name = s.next().unwrap();
            let focal_len = s.next().unwrap().parse::<usize>().unwrap();
            let box_idx = hash(name);
            if let Some(pos) = boxes[box_idx].iter().position(|l| *l.chars == *name) {
                boxes[box_idx][pos] = Lens {
                    chars: name,
                    focal_length: focal_len,
                };
            } else {
                boxes[box_idx].push(Lens {
                    chars: name,
                    focal_length: focal_len,
                });
            }
        } else {
            let name = &s[..s.len() - 1];
            let box_idx = hash(name);
            if let Some(pos) = boxes[box_idx].iter().position(|l| *l.chars == *name) {
                boxes[box_idx].remove(pos);
            }
        }
    });

    let res: usize = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(|(slot_idx, lens)| (box_idx + 1) * (slot_idx + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum();
    println!("{}", res);
}
