use std::collections::HashMap;


fn find_first_digit(input: &str) -> u32 {
    let lut: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    let (min_letter_pos, letter_digit)= lut.iter()
        .map(|(&word, &digit)|
            match input.find(word) {
                None => (input.len(), 0u32),
                Some(pos) => (pos, digit)
            })
        .min().unwrap();

    if let Some(min_digit_pos) = input.chars().position(|c| c.is_digit(10)){
        if min_digit_pos < min_letter_pos {
            return char::to_digit(input.chars().nth(min_digit_pos).unwrap(), 10).unwrap();
        }
    }
    return letter_digit;
}

fn find_last_digit(input: &str) -> u32 {
    let lut: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    let (max_letter_pos, letter_digit)= lut.iter()
        .map(|(&word, &digit)|
            match input.rfind(word) {
                None => (0, 0u32),
                Some(pos) => (pos, digit)
            })
        .max().unwrap();

    if let Some(max_digit_pos) = input.chars().rev().position(|c| c.is_digit(10)){
        if input.len() - max_digit_pos > max_letter_pos {
            return char::to_digit(input.chars().rev().nth(max_digit_pos).unwrap(), 10).unwrap();
        }
    }
    return letter_digit;
}

fn main() {
    let mut sum: u64= 0;
    let program_name = std::env::args().nth(0).expect("there are no args");
    if std::env::args().len() < 2 {
        eprintln!("Usage: {} <input_file>", program_name);
    }

    let input_file = std::env::args()
        .nth(1)
        .expect("Expected filename as argument");

    for line in std::fs::read_to_string(input_file).unwrap().lines() {
        let first_digit = find_first_digit(line);
        let last_digit = find_last_digit(line);
        sum += first_digit as u64 * 10 + last_digit as u64;
    }
    print!("{}", sum);
}
