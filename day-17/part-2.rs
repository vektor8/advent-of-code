use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    current: (i32, i32),
    direction: (i32, i32),
    steps: u32,
    heat_loss: u32,
}

#[derive(Hash, PartialEq, Eq)]
struct VisitedState {
    current: (i32, i32),
    direction: (i32, i32),
    steps: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(input: &Vec<Vec<u32>>) -> u32 {
    let mut min_heap: BinaryHeap<State> = BinaryHeap::new();
    min_heap.push(State {
        current: (0, 0),
        direction: (0, 0),
        steps: 0,
        heat_loss: 0,
    });
    let mut visited: HashSet<VisitedState> = HashSet::new();
    while let Some(state) = min_heap.pop() {
        if state.current == ((input.len() - 1) as i32, (input[0].len() - 1) as i32) {
            return state.heat_loss;
        }

        let key = VisitedState {
            current: state.current,
            direction: state.direction,
            steps: state.steps,
        };
        if visited.contains(&key) {
            continue;
        }

        visited.insert(key);

        if state.steps < 10 && state.direction != (0, 0) {
            let next_pos = (
                state.current.0 + state.direction.0,
                state.current.1 + state.direction.1,
            );

            if next_pos.0 >= 0
                && next_pos.0 < input.len() as i32
                && next_pos.1 >= 0
                && next_pos.1 < input[0].len() as i32
            {
                min_heap.push(State {
                    current: next_pos,
                    direction: state.direction,
                    steps: state.steps + 1,
                    heat_loss: state.heat_loss + input[next_pos.0 as usize][next_pos.1 as usize],
                });
            }
        }

        if state.steps < 4 && state.direction != (0, 0) {
            continue;
        }

        for direction in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            if direction == state.direction || direction == (-state.direction.0, -state.direction.1)
            {
                continue;
            }
            let next_pos = (state.current.0 + direction.0, state.current.1 + direction.1);

            if next_pos.0 >= 0
                && next_pos.0 < input.len() as i32
                && next_pos.1 >= 0
                && next_pos.1 < input[0].len() as i32
            {
                min_heap.push(State {
                    current: next_pos,
                    direction,
                    steps: 1,
                    heat_loss: state.heat_loss + input[next_pos.0 as usize][next_pos.1 as usize],
                });
            }
        }
    }
    0
}
fn main() {
    let input: Vec<Vec<u32>> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let res = dijkstra(&input);
    println!("{}", res);
}
