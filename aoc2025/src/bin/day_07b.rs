use std::fs;
use nalgebra::DMatrix;

const PATH: &str = "inputs/day_07.txt";

fn main() {
    let mut data_matrix = process_file(PATH);

    for row in 0..data_matrix.shape().0 - 1 {
        process_matrix_row(&mut data_matrix, row);
    }
    let answer_b = calculate_number_of_timelines(&data_matrix);
    println!("Result B: {}", answer_b);
}


fn calculate_number_of_timelines(data_matrix: &DMatrix<i64>) -> i64 {
    // summing last row
    data_matrix.row(data_matrix.shape().0 - 1)
        .iter()
        .filter(|&&x| x >= 0)
        .sum()
}

fn process_matrix_row(data_matrix: &mut DMatrix<i64>, row: usize) {
    let n_cols = data_matrix.shape().1;

    for col in 0..n_cols {
        if data_matrix[(row, col)] > 0 {
            extends_tachyon(data_matrix, row, col);
        }
    }
}

fn extends_tachyon(data_matrix: &mut DMatrix<i64>, row: usize, col: usize) {
    let n_timelines = data_matrix[(row, col)];

    if data_matrix[(row + 1, col)] >= 0 {
        // moving tachyom down
        data_matrix[(row + 1, col)] += n_timelines;
        return;
    }

    if data_matrix[(row + 1, col)] == -1 {
        // splitting tachyom
        if data_matrix[(row + 1, col.saturating_sub(1))] >= 0 {
            // left side
            data_matrix[(row + 1, col.saturating_sub(1))] += n_timelines;
        }

        if col + 1 < data_matrix.shape().1 && data_matrix[(row + 1, col + 1)] >= 0 {
            // right side
            data_matrix[(row + 1, col + 1)] += n_timelines;
        }
    }
}

fn mapping_fn(symbol: char) -> i64 {
    match symbol {
        'S' => 1,
        '.' => 0,
        '^' => -1,
        _ => panic!("Unknown symbol: {}", symbol),
    }
}

fn process_file(path: &str) -> DMatrix<i64> {
    let contents = fs::read_to_string(path).unwrap();

    let n_rows: usize = contents.lines().count();
    let n_cols: usize = contents.lines().next().unwrap().len();

    let temp_char_vec: Vec<i64> = contents.trim().chars()
        .filter(|x| *x != '\n')
        .map(|symbol| mapping_fn(symbol))
        .collect();

    DMatrix::from_row_slice(n_rows, n_cols, &temp_char_vec)
}