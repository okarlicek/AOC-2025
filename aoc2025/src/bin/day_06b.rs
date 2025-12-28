use std::cmp::max;
use std::fs;

const PATH: &str = "inputs/day_06.txt";

fn main() {
    let mut operators: &mut Vec<char> = &mut Vec::new();
    let mut numbers = &mut Vec::<u64>::new();
    process_file(PATH, &mut numbers, &mut operators);

    let mut result_b = 0;
    let mut temp_result: u64;

    let mut it_numbers = numbers.iter().peekable();

    for op in operators {
        temp_result = 0;
        while let Some(x) = it_numbers.peek() {
            if **x == 0 { it_numbers.next(); break; }

            match op {
                '+' => temp_result += *x,
                '*' => temp_result = max(temp_result, 1) * *x,
                _ => println!("Wrong operator!"),
            }
            it_numbers.next();
        }

        result_b += temp_result;
    }


    println!("Result B: {}", result_b);
}


fn process_file(path: &str, numbers: &mut Vec::<u64>, operators: &mut Vec<char>) {
    let contents = fs::read_to_string(path).unwrap();

    let mut first_line = true;
    let mut i: usize;

    for line in contents.lines() {
        i = 0;

        for symbol in line.chars() {
            if first_line {
                numbers.push(0);
            }
            if symbol == ' ' {
                i += 1; continue;
            }
            if matches! (symbol, '+' | '*') {
                operators.extend(line.chars().filter(|x| !x.is_ascii_whitespace()).collect::<Vec<char>>());
                break;
            }

            numbers[i] = numbers[i] * 10 +  symbol.to_digit(10).unwrap() as u64;
            i+= 1
        }
        first_line = false;
    }
}