use std::collections::HashMap;

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

    let mut state = "AAA";
    let mut step = 0;
    while state != "ZZZ" {
        state = match instructions[step % instructions.len()] {
            'R' => states_changes.get(state).unwrap().1,
            'L' => states_changes.get(state).unwrap().0,
            i => panic!("Unknown instruction in sequence: {}", i),
        };
        step += 1;
    }

    println!("{}", step);
}
