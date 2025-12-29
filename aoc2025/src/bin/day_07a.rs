use std::fs;
use nalgebra::DMatrix;

const PATH: &str = "inputs/day_07_test.txt";

fn main() {
    let mut data_matrix = process_file(PATH);
    let mut answer_a = 0;

    for row in 0..data_matrix.shape().0 - 1 {
        answer_a += process_matrix_row(&mut data_matrix, row);
    }

    println!("Result A: {}", answer_a);
}

fn process_matrix_row(data_matrix: &mut DMatrix<char>, row: usize) -> u32 {
    let n_cols = data_matrix.shape().1;
    let mut n_splits = 0;

    for col in 0..n_cols {
        match data_matrix[(row, col)] {
            'S' | '|' => n_splits += extends_tachyon(data_matrix, row, col),
            _ => continue,
        }
    }

    return n_splits;
}

fn extends_tachyon(data_matrix: &mut DMatrix<char>, row: usize, col: usize) -> u32 {
    if data_matrix[(row + 1, col)] == '.' {
        // moving tachyom down
        data_matrix[(row + 1, col)] = '|';
        return 0;
    }

    if data_matrix[(row + 1, col)] == '^' {
        // splitting tachyom
        if data_matrix[(row + 1, col.saturating_sub(1))] == '.' {
            // left side
            data_matrix[(row + 1, col.saturating_sub(1))] = '|';
        }

        if col + 1 < data_matrix.shape().1 && data_matrix[(row + 1, col + 1)] == '.' {
            // right side
            data_matrix[(row + 1, col + 1)] = '|';
        }
        return 1;
    }

    return 0;
}

fn process_file(path: &str) -> DMatrix<char> {
    let contents = fs::read_to_string(path).unwrap();

    let n_rows: usize = contents.lines().count();
    let n_cols: usize = contents.lines().next().unwrap().len();

    let temp_char_vec: Vec<char> = contents.trim().chars().filter(|x| *x != '\n').collect();
    return DMatrix::from_row_slice(n_rows, n_cols, &temp_char_vec);
}