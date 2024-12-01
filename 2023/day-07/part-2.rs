use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Card {
    CJ = 0,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

fn char_to_card(c: char) -> Card {
    match c {
        '2' => Card::C2,
        '3' => Card::C3,
        '4' => Card::C4,
        '5' => Card::C5,
        '6' => Card::C6,
        '7' => Card::C7,
        '8' => Card::C8,
        '9' => Card::C9,
        'T' => Card::CT,
        'J' => Card::CJ,
        'Q' => Card::CQ,
        'K' => Card::CK,
        'A' => Card::CA,
        _ => Card::CA,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn transform_hand(hand: Vec<Card>) -> Vec<Card> {
    let most_frequent = hand
        .iter()
        .filter(|&&x| x != Card::CJ)
        .max_by_key(|x| hand.iter().filter(|a| x == a).count())
        .unwrap_or(&Card::C2);
    let ret = hand
        .iter()
        .map(|&x| match x {
            Card::CJ => *most_frequent,
            _ => x,
        })
        .collect();
    // println!("{:?} {:?}", hand, ret);
    ret
}
fn eval_hand(hand: &Vec<Card>) -> HandType {
    let mut count_map: HashMap<Card, usize> = HashMap::new();

    // Iterate over the elements of the vector and update the count in the HashMap
    for element in hand {
        let count = count_map.entry(*element).or_insert(0);
        *count += 1;
    }

    match count_map.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if count_map.iter().any(|(_, &v)| v == 1) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if count_map.iter().any(|(_, &v)| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }

        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn main() {
    let mut input: Vec<(Vec<Card>, u32)> = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|line| {
            let hand = line
                .split(' ')
                .next()
                .unwrap()
                .chars()
                .map(char_to_card)
                .collect();
            let bid = line.split(' ').nth(1).unwrap().parse::<u32>().unwrap();
            (hand, bid)
        })
        .collect();

    input.sort_by(|a, b| {
        let hand_a = eval_hand(&transform_hand(a.0.clone()));
        let hand_b = eval_hand(&transform_hand(b.0.clone()));
        if hand_a != hand_b {
            return hand_a.cmp(&hand_b);
        }
        for (x, y) in a.0.iter().zip(b.0.iter()) {
            if x != y {
                return x.cmp(y);
            }
        }
        std::cmp::Ordering::Equal
    });
    let res: u32 = input
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) as u32 * bid)
        .sum();
    println!("{}", res);
}
