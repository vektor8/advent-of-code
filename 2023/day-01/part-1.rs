fn main() {
    let mut sum = 0;
    let program_name = std::env::args().nth(0).expect("there are no args");
    if std::env::args().len() < 2 {
        eprintln!("Usage: {} <input_file>", program_name);
    }

    let input_file = std::env::args().nth(1).expect("Expected filename as argument");

    for line in std::fs::read_to_string(input_file).unwrap().lines() {
        let mut first_digit = '0';
        let mut second_digit = '0';
        for c in line.chars(){
            if c.is_digit(10){
                first_digit = c;
                break;
            }
        }
        
        for c in line.chars().rev(){
            if c.is_digit(10){
                second_digit = c;
                break;
            }
        }
        
        sum += char::to_digit(first_digit, 10).unwrap() * 10 + char::to_digit(second_digit, 10).unwrap();
    }
    print!("{}", sum);
}
