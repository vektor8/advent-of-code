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
fn simulate(modules: HashMap<String, Module>) -> (HashMap<String, Module>, usize, usize) {
    let mut modules = modules.clone();
    let mut q: VecDeque<(String, bool, String)> =
        VecDeque::from([(String::from("none"), false, String::from("broadcaster"))]);
    let (mut low_count, mut high_count) = (0, 0);
    while !q.is_empty() {
        let (sender, pulse, module_name) = q.pop_front().unwrap();
        match pulse {
            true => low_count += 1,
            false => high_count += 1,
        };
        let module = match modules.get(&module_name) {
            Some(m) => m,
            None => {
                continue;
            }
        };
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
    (modules, low_count, high_count)
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
    let mut modules = modules
        .iter()
        .map(|(k, v)| {
            if v.m_type == ModuleType::Conjuction {
                (k.clone(), add_dependencies(v.clone(), k, &modules_copy))
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect();
    let (mut low_count, mut high_count) = (0, 0);
    for _ in 0..1000 {
        let (next, l, h) = simulate(modules);
        low_count += l;
        high_count += h;
        modules = next;
    }
    println!("{}", low_count * high_count)
}
