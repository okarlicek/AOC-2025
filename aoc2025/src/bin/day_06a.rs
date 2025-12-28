use std::fs;
use std::str::FromStr;
use nalgebra::DMatrix;

const PATH: &str = "inputs/day_06.txt";

fn main() {
    let mut operators: &mut Vec<char> = &mut Vec::new();
    let numbers: DMatrix<u64> = process_file(PATH, &mut operators);

    let mut result_a = 0;
    for (i, op) in operators.iter().enumerate() {
        if *op == '+' {
            result_a += numbers.column(i).sum();
        } else if *op == '*' {
            result_a += numbers.column(i).product();
        }
    }

    println!("Result A: {}", result_a);
}


fn process_file(path: &str, operators: &mut Vec<char>) -> DMatrix<u64> {
    let contents = fs::read_to_string(path).unwrap();

    let mut n_rows: usize = 0;
    let mut n_cols: usize = 0;
    let mut temp_numbers = Vec::<u64>::new();

    for line in contents.lines() {
        n_rows += 1;
        for symbol in line.split_whitespace() {
            if symbol == "*" || symbol == "+" {
                n_cols += 1;
                operators.push(char::from_str(symbol).unwrap());
            } else {
                temp_numbers.push(u64::from_str(symbol).unwrap());
            }
        }
    }
    println!("ncols: {}, nrows: {}", n_cols, n_rows);
    return DMatrix::from_row_slice(n_rows.saturating_sub(1), n_cols, &temp_numbers);
}