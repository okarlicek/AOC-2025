use std::fs;
use nalgebra::{min, DMatrix};

const PATH: &str = "./inputs/day_04.txt";

fn main() {
    let mut input_matrix: DMatrix<i32>= create_matrix_from_path(PATH);

    let (n_rows, n_cols) = input_matrix.shape();
    let neighbours_matrix = &mut DMatrix::<i32>::zeros(n_rows, n_cols);

    update_neighbours_counts(&input_matrix, neighbours_matrix);
    let mut removable_count = neighbours_matrix.iter().filter(|&&x| x >= 0 && x < 4).count();
    remove_items_from_input_matrix(&mut input_matrix, neighbours_matrix);

    let answer_a = removable_count;
    let mut answer_b = removable_count;

    while removable_count > 0 {
        update_neighbours_counts(&input_matrix, neighbours_matrix);
        removable_count = neighbours_matrix.iter().filter(|&&x| x >= 0 && x < 4).count();
        remove_items_from_input_matrix(&mut input_matrix, neighbours_matrix);

        answer_b += removable_count;
    }

    remove_items_from_input_matrix(&mut input_matrix, neighbours_matrix);
    println!("Answer A: {}", answer_a);
    println!("Answer B: {}", answer_b);
}


fn remove_items_from_input_matrix(
    input_matrix: &mut DMatrix<i32>, neighbours_matrix: &DMatrix<i32>,
) {
    let (n_rows, n_cols) = input_matrix.shape();

    for i in 0..n_rows {
        for j in 0..n_cols {
            if neighbours_matrix[(i, j)] >= 0 && neighbours_matrix[(i, j)] < 4 {
                input_matrix[(i, j)] = 0;
            }
        }
    }
}

fn update_neighbours_counts(
    input_matrix: &DMatrix<i32>, neighbours_matrix: &mut DMatrix<i32>,
) {
    let (n_rows, n_cols) = input_matrix.shape();

    for i in 0..n_rows {
        for j in 0..n_cols {
            if input_matrix[(i, j)] == 0 {
                neighbours_matrix[(i, j)] = -1;
            } else {
                let start_row = i.saturating_sub(1);
                let start_col = j.saturating_sub(1);

                let view_rows = min(i + 1, n_rows - 1) - start_row + 1;
                let view_cols = min(j + 1, n_cols - 1) - start_col + 1;

                let view_sum = input_matrix.view(
                    (start_row, start_col),
                    (view_rows, view_cols)
                ).sum();

                // - 1 for the element it self
                neighbours_matrix[(i, j)] = view_sum - 1;
            }

        }
    }
}


fn create_matrix_from_path(path: &str) -> DMatrix<i32> {
    let mut n_rows = 0;
    let mut n_cols = 0;

    let contents = fs::read_to_string(path);
    let mut contents_vec_int = Vec::<i32>::with_capacity(n_rows * n_cols);

    for line in contents.unwrap().lines() {
        n_rows += 1;
        n_cols = line.len();

        for c in line.chars() {
            if c == '.' {
                contents_vec_int.push(0);
            } else if c == '@' {
                contents_vec_int.push(1);
            }
        }
    }

   return DMatrix::from_row_slice(n_rows, n_cols, &contents_vec_int);
}