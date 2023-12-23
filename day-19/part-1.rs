use std::collections::HashMap;
#[derive(Debug)]
enum Comparison {
    Less,
    Greater,
}

#[derive(Debug)]
enum Rule {
    Condition((char, Comparison, u64, String)),
    Final(String),
}
#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

fn is_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = "in";
    loop {
        match current {
            "A" => return true,
            "R" => return false,
            _ => {}
        };
        for rule in &workflows.get(current).unwrap().rules {
            match rule {
                Rule::Condition((field, cmp, value, next)) => {
                    let part_value = match field {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        _ => panic!("No such field"),
                    };
                    match cmp {
                        Comparison::Less => {
                            if part_value < *value {
                                current = next;
                                break;
                            }
                        }
                        Comparison::Greater => {
                            if part_value > *value {
                                current = next;
                                break;
                            }
                        }
                    }
                }
                Rule::Final(w) => {
                    current = w;
                    break;
                }
            }
        }
    }
}
fn main() {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    let file = std::fs::read_to_string("input").expect("Input file not found");
    let mut input = file.split("\n\n");

    input.next().unwrap().lines().for_each(|line| {
        let name = line.split('{').next().unwrap();
        let rule_section = line.split('{').nth(1).unwrap();
        let rule: Vec<Rule> = rule_section
            .split(',')
            .map(|rule| {
                if !rule.contains(':') {
                    return Rule::Final(rule[..rule.len() - 1].to_owned());
                }
                let c = rule.chars().next().unwrap();
                let cmp = match rule.chars().nth(1).unwrap() {
                    '<' => Comparison::Less,
                    '>' => Comparison::Greater,
                    _ => panic!("Unknown"),
                };
                let no = rule[2..].split(':').next().unwrap().parse::<u64>().unwrap();
                let next = rule[2..].split(':').nth(1).unwrap();
                Rule::Condition((c, cmp, no, next.to_owned()))
            })
            .collect();
        workflows.insert(name.to_owned(), Workflow { rules: rule });
    });

    println!("{:?}", workflows);

    let parts: Vec<Part> = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let line = &line[1..line.len() - 1];
            let numbers: Vec<u64> = line
                .split(',')
                .map(|e| e.split('=').nth(1).unwrap().parse::<u64>().unwrap())
                .collect();
            Part {
                x: numbers[0],
                m: numbers[1],
                a: numbers[2],
                s: numbers[3],
            }
        })
        .collect();
    let res: u64 = parts
        .iter()
        .filter(|p| is_accepted(p, &workflows))
        .map(|p| p.x + p.m + p.a + p.s)
        .sum();
    println!("{}", res);
}
