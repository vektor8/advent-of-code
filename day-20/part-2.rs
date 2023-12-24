use std::collections::{HashMap, VecDeque};

type Pulse = bool;

#[derive(Debug, PartialEq, Clone)]
enum ModuleType {
    FlipFlop,
    Conjuction,
    Broadcaster,
}

#[derive(Debug, Clone)]
struct Module {
    m_type: ModuleType,
    state: Pulse,
    receivers: Vec<String>,
    dependencies: HashMap<String, Pulse>,
}

fn add_dependencies(module: Module, name: &String, modules: &HashMap<String, Module>) -> Module {
    let dependencies = modules
        .iter()
        .filter(|(_, v)| v.receivers.contains(name))
        .map(|(k, _)| (k.clone(), false))
        .collect();
    let mut module = module.clone();
    module.dependencies = dependencies;
    module
}
fn simulate(
    modules: HashMap<String, Module>,
    feed: &String,
    cycles: &mut HashMap<String, usize>,
    count: usize,
) -> HashMap<String, Module> {
    let mut modules = modules.clone();
    let mut q: VecDeque<(String, bool, String)> =
        VecDeque::from([(String::from("none"), false, String::from("broadcaster"))]);
    while !q.is_empty() {
        let (sender, pulse, module_name) = q.pop_front().unwrap();
        let module = match modules.get(&module_name) {
            Some(m) => m,
            None => {
                continue;
            }
        };

        if module_name == *feed && pulse && *cycles.get(&sender).unwrap() == 0 {
            cycles.insert(sender.clone(), count);
        }

        match module.m_type {
            ModuleType::FlipFlop => match pulse {
                false => {
                    let mut new = module.clone();
                    new.state = !module.state;
                    for r in new.receivers.iter() {
                        q.push_back((module_name.clone(), new.state, r.clone()))
                    }
                    modules.insert(module_name, new);
                }
                true => {}
            },
            ModuleType::Conjuction => {
                let mut new = module.clone();
                new.dependencies.insert(sender, pulse);
                if new.dependencies.values().all(|f| *f) {
                    for r in module.receivers.iter() {
                        q.push_back((module_name.clone(), false, r.clone()))
                    }
                } else {
                    for r in module.receivers.iter() {
                        q.push_back((module_name.clone(), true, r.clone()))
                    }
                }
                modules.insert(module_name, new);
            }
            ModuleType::Broadcaster => {
                for r in module.receivers.iter() {
                    q.push_back((module_name.clone(), pulse, r.clone()));
                }
            }
        }
    }
    modules
}
fn main() {
    let modules: HashMap<String, Module> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| {
            let first_char = line.chars().next().unwrap();
            let line = &line[1..];
            let name = line.split(" -> ").next().unwrap();
            let receivers: Vec<String> = line
                .split(" -> ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|e| e.to_owned())
                .collect();
            match first_char {
                '&' => {
                    let m = Module {
                        m_type: ModuleType::Conjuction,
                        state: false,
                        receivers,
                        dependencies: HashMap::new(),
                    };
                    (name.to_owned(), m)
                }
                '%' => {
                    let m = Module {
                        m_type: ModuleType::FlipFlop,
                        state: false,
                        receivers,
                        dependencies: HashMap::new(),
                    };
                    (name.to_owned(), m)
                }
                'b' => {
                    let m = Module {
                        m_type: ModuleType::Broadcaster,
                        state: false,
                        receivers,
                        dependencies: HashMap::new(),
                    };
                    (String::from("broadcaster"), m)
                }
                _ => panic!("Unknown module"),
            }
        })
        .collect();

    let modules_copy = modules.clone();
    let mut modules: HashMap<String, Module> = modules
        .iter()
        .map(|(k, v)| {
            if v.m_type == ModuleType::Conjuction {
                (k.clone(), add_dependencies(v.clone(), k, &modules_copy))
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect();

    let feed = &modules
        .iter()
        .find(|(_, m)| m.receivers.contains(&String::from("rx")))
        .unwrap()
        .0
        .clone()
        .clone();
    let mut cycles: HashMap<String, usize> = modules
        .get(feed)
        .unwrap()
        .dependencies
        .keys()
        .map(|k| (k.clone(), 0))
        .collect();
    let mut i = 0;
    loop {
        i += 1;
        let next = simulate(modules, feed, &mut cycles, i);
        if cycles.values().all(|v| *v > 0) {
            break;
        }
        modules = next;
    }

    let mut res = 1;
    for v in cycles.values() {
        res = lcm(res, *v);
    }
    println!("{}", res);
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
