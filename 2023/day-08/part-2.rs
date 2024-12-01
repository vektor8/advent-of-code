use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn main() {
    let file = std::fs::read_to_string("input").expect("Input file not found");
    let lines: Vec<&str> = file.lines().collect();

    let instructions: Vec<char> = lines[0].chars().collect();

    let mut states_changes: HashMap<&str, (&str, &str)> = HashMap::new();

    let lines: Vec<String> = lines.iter().skip(2).map(|l| l.replace(' ', "")).collect();

    lines.iter().for_each(|line| {
        let key = line.split('=').next().unwrap();
        let value = line.split('=').nth(1).unwrap();
        let lvalue = &value.split(',').next().unwrap()[1..];
        let rvalue = &value.split(',').nth(1).unwrap();
        let rvalue = &rvalue[..rvalue.len() - 1];

        states_changes.insert(key, (lvalue, rvalue));
    });

    let states: Vec<&str> = states_changes
        .iter()
        .map(|(&k, _)| k)
        .filter(|k| k.ends_with('A'))
        .collect();

    let step_counts: Vec<usize> = states
        .iter()
        .map(|&s| {
            let mut s = s;
            let mut step = 0;
            while !s.ends_with('Z') {
                s = match instructions[step % instructions.len()] {
                    'R' => states_changes.get(s).unwrap().1,
                    'L' => states_changes.get(s).unwrap().0,
                    i => panic!("Unknown instruction in sequence: {}", i),
                };
                step += 1;
            }
            step
        })
        .collect();

    let result = step_counts.iter().cloned().fold(1, lcm);
    println!("{}", result);
}
