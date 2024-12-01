struct Mapping {
    destination: u64,
    start: u64,
    range: u64,
}

impl Mapping {
    fn map_number(&self, x: u64) -> Option<u64> {
        if x >= self.start && x < self.start + self.range {
            Some(x - self.start + self.destination)
        } else {
            None
        }
    }
}

struct MappingGroup {
    mappers: Vec<Mapping>,
}

impl MappingGroup {
    fn map_number(&self, x: u64) -> u64 {
        for mapper in self.mappers.iter() {
            if let Some(res) = mapper.map_number(x) {
                return res;
            }
        }
        x
    }
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let seeds: Vec<u64> = input[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut idx = 3;
    let mut groups: Vec<MappingGroup> = vec![];
    while idx < input.len() {
        let mut group = MappingGroup { mappers: vec![] };
        while idx < input.len() && !input[idx].is_empty() {
            let mut it = input[idx].split(' ');
            group.mappers.push(Mapping {
                destination: it.next().unwrap().parse::<u64>().unwrap(),
                start: it.next().unwrap().parse::<u64>().unwrap(),
                range: it.next().unwrap().parse::<u64>().unwrap(),
            });
            idx += 1;
        }
        groups.push(group);
        idx += 2;
    }
    assert!(seeds.len() % 2 == 0);
    let res = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .map(|(seed, range)| {
            (seed..(seed + range)).map(|i| {
                let mut i = i;
                for group in groups.iter() {
                    i = group.map_number(i);
                }
                i
            })
        })
        .flat_map(|v| v.into_iter())
        .min()
        .unwrap();
    println!("{:?}", res);
}
