use std::fs;

const PATH: &str = "./inputs/day_02.txt";
struct Indices {
    start: i64,
    end: i64,
}

fn main() {
    let mut vec_indices: Vec<Indices> = Vec::new();
    let mut invalid_ids: Vec<i64> = Vec::new();

    get_indices_from_path(PATH, &mut vec_indices);

    for index in vec_indices {
        get_invalid_ids_in_range(index.start, index.end, &mut invalid_ids);
    }

    let answer_one: i64 = invalid_ids.iter().sum();

    println!("The answer to problem 1 is {}", answer_one);
}

fn get_indices_from_path(path: &str, vec_indices: &mut Vec<Indices>) {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    for indices_split in contents.split(",") {
        let (start, end) = indices_split.split_once('-').unwrap();
        println!("start: {}, end: {}", start, end);
        vec_indices.push(Indices{
            start: str::parse(start).unwrap(),
            end: str::parse(end).unwrap()
        });
    }
}

fn get_first_half_of_the_number(number: i64, use_quotient: bool) -> i64 {
    let number_string: String = number.to_string();
    let number_length = number_string.len();

    if number_length == 1 {
        return 1;
    }

    let mut half_index = number_length / 2;

    if use_quotient {
        let quotient = number_length % 2;

        half_index += quotient;
    }

    return number_string[0..half_index].parse::<i64>().expect("Could not parse number");
}

fn create_invalid_id_from_half(number: i64) -> i64 {
    return format!("{number}{number}").parse::<i64>().expect("Could not parse number");
}

fn get_invalid_ids_in_range(start: i64, end: i64, invalid_ids: &mut Vec<i64>) {
    let start_half: i64 = get_first_half_of_the_number(start, false);
    let end_half: i64 = get_first_half_of_the_number(end, true);
    println!("start_half: {}", start_half);
    println!("end_half: {}", end_half);
    for number in start_half..=end_half {
        let invalid_id = create_invalid_id_from_half(number);

        if (start <= invalid_id) && (invalid_id <= end) {
            invalid_ids.push(invalid_id);
        }
    }
}