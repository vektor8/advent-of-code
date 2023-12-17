use std::collections::HashMap;

fn search<'a>(
    chars: &'a [char],
    expected: &'a [usize],
    cache: &mut HashMap<(&'a [char], &'a [usize]), usize>,
) -> usize {
    if chars.is_empty() {
        if expected.is_empty() {
            return 1;
        }
        return 0;
    }

    if expected.is_empty() {
        if chars.contains(&'#') {
            return 0;
        }
        return 1;
    }
    let key = (chars, expected);
    if let Some(&res) = cache.get(&key) {
        return res;
    }

    let mut ret = 0;
    if ".?".contains(chars[0]) {
        ret += search(&chars[1..], expected, cache);
    }

    if "#?".contains(chars[0])
        && expected[0] <= chars.len()
        && !chars[..expected[0]].contains(&'.')
        && (expected[0] == chars.len() || chars[expected[0]] != '#')
    {
        if expected[0] + 1 > chars.len() {
            ret += search(&[], &expected[1..], cache);
        } else {
            ret += search(&chars[(expected[0] + 1)..], &expected[1..], cache);
        }
    }
    cache.insert(key, ret);
    ret
}
fn main() {
    let res: usize = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split(' ').collect();
            let (chars, nums) = (l[0], l[1]);
            let mut chars: Vec<char> = chars.chars().collect();
            let mut nums: Vec<usize> = nums
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let orig_chars = chars.clone();
            let orig_nums = nums.clone();

            for _ in 0..4 {
                chars.push('?');
                chars.extend(&orig_chars);
                nums.extend(&orig_nums);
            }
            let mut cache = HashMap::new();
            search(&chars, &nums, &mut cache)
        })
        .sum();

    println!("{}", res);
}
