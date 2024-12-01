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
fn find_accepted(
    workflow: &String,
    part: &mut HashMap<char, (u64, u64)>,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    match workflow.as_str() {
        "A" => {
            return part.values().map(|(a, b)| *b - *a + 1).product();
        }
        "R" => return 0,
        _ => {}
    }

    let mut res = 0;
    for rule in &workflows.get(workflow).unwrap().rules {
        match rule {
            Rule::Condition((field, cmp, value, next)) => {
                let part_value = *part.get(field).unwrap();
                match cmp {
                    Comparison::Less => {
                        if part_value.0 >= *value {
                            continue;
                        } else if part_value.1 < *value {
                            res += find_accepted(next, part, workflows);
                            break;
                        } else {
                            let new_part_value = (part_value.0, *value - 1);
                            let mut copy = part.clone();
                            copy.insert(*field, new_part_value);
                            res += find_accepted(next, &mut copy, workflows);
                            part.insert(*field, (*value, part_value.1));
                        }
                    }
                    Comparison::Greater => {
                        if part_value.1 <= *value {
                            continue;
                        } else if part_value.0 > *value {
                            res += find_accepted(next, part, workflows);
                            break;
                        } else {
                            let new_part_value = (*value + 1, part_value.1);
                            let mut copy = part.clone();
                            copy.insert(*field, new_part_value);
                            res += find_accepted(next, &mut copy, workflows);
                            part.insert(*field, (part_value.0, *value));
                        }
                    }
                }
            }
            Rule::Final(w) => res += find_accepted(w, part, workflows),
        }
    }
    res
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

    let res = find_accepted(
        &"in".to_string(),
        &mut HashMap::from([
            ('x', (1, 4000)),
            ('m', (1, 4000)),
            ('a', (1, 4000)),
            ('s', (1, 4000)),
        ]),
        &workflows,
    );
    println!("{}", res);
}
