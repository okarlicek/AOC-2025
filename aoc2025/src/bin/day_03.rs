use std::fs;

const PATH: &str = "./inputs/day_03.txt";


fn main() {
    let mut joltages_problem_a: Vec<i64> = Vec::new();
    let mut joltages_problem_b: Vec<i64> = Vec::new();
    let contents = fs::read_to_string(PATH)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        joltages_problem_a.push(get_maximum_joltage_per_size(&line, 2));
        joltages_problem_b.push(get_maximum_joltage_per_size(&line, 12));
    }

    let asnwer_one: i64 = joltages_problem_a.iter().sum();
    let asnwer_two: i64 = joltages_problem_b.iter().sum();

    println!("Answer one: {}", asnwer_one);
    println!("Answer two: {}", asnwer_two);
}

fn get_maximum_joltage_per_size(battery_str: &str, size: i64) -> i64 {
    let mut used_batteries: Vec<i64> = vec![0; size as usize];
    let battery_length = battery_str.len();

    for (i, b) in battery_str.chars().enumerate() {
        let b_int: i64 = b.to_string().parse().unwrap();

        let mut min_index_temp: i64 = size - (battery_length as i64 - i as i64);
        if  min_index_temp < 0 { min_index_temp = 0; }

        let min_index: usize = min_index_temp as usize;

        for index in min_index..size as usize {
            if used_batteries[index] < b_int {
                used_batteries[index] = b_int;

                // null the rest of the vector so we can add the remaining batteries
                for null_index in index + 1..size as usize {
                    used_batteries[null_index] = 0;
                }
                break;
            }
        }
    }

    let mut result: i64 = 0;
    let mut pow_coef: u32 = size as u32;

    for b in used_batteries {
        pow_coef -= 1;
        result += b * i64::pow(10, pow_coef);
    }
    return result;
}