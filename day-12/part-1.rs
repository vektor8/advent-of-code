fn is_solution(sol: &[char], expected: &[usize]) -> bool {
    let str: String = sol.iter().collect();
    let vec: Vec<usize> = str
        .split('.')
        .filter(|s| !s.is_empty())
        .map(|l| l.len())
        .collect();
    vec == expected
}

fn search(chars: &mut Vec<char>, expected: &[usize]) -> usize {
    if chars.iter().all(|&c| c != '?') {
        if is_solution(chars, expected) {
            return 1;
        }
        return 0;
    }

    let pos = chars.iter().position(|&c| c == '?').unwrap();
    chars[pos] = '.';
    let mut ret = 0;
    ret += search(chars, expected);
    chars[pos] = '#';
    ret += search(chars, expected);
    chars[pos] = '?';
    ret
}
fn main() {
    let res: usize = std::fs::read_to_string("input")
        .expect("Input file not found")
        .lines()
        .map(|l| {
            let l: Vec<&str> = l.split(' ').collect();
            let (chars, nums) = (l[0], l[1]);
            let mut chars = chars.chars().collect();
            let nums: Vec<usize> = nums
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            search(&mut chars, &nums)
        })
        .sum();

    println!("{}", res);
}
