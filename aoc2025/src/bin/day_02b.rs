use std::fs;

const PATH: &str = "./inputs/day_02.txt";
struct Indices {
    start: i64,
    end: i64,
}
struct RepetitionPattern {
    start: i64,
    end: i64,
    repetitions: i64,
}

fn main() {
    let mut vec_indices: Vec<Indices> = Vec::new();
    let mut invalid_ids: Vec<i64> = Vec::new();

    get_indices_from_path(PATH, &mut vec_indices);

    for index in vec_indices {
        get_invalid_ids_in_range(index.start, index.end, &mut invalid_ids);
    }

    let answer_two: i64 = invalid_ids.iter().sum();

    println!("The answer to problem 2 is {}", answer_two);

}

fn get_indices_from_path(path: &str, vec_indices: &mut Vec<Indices>) {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    for indices_split in contents.split(",") {
        let (start, end) = indices_split.split_once('-').unwrap();

        vec_indices.push(Indices{
            start: str::parse(start).unwrap(),
            end: str::parse(end).unwrap()
        });
    }
}

fn get_repetition_pattern(start: i64, end: i64, vec_of_repetitions: &mut Vec<RepetitionPattern>) {
    let start_string: String = start.to_string();
    let start_length = start_string.len();

    let end_string: String = end.to_string();
    let end_length = end_string.len();

    for pattern_size in 1..=end_length / 2 + end_length % 2 {
        let pattern_start = i64::pow(10, pattern_size as u32) / 10;
        let pattern_end = i64::pow(10, pattern_size as u32) - 1;

        let repetition_start = start_length / pattern_size;
        let mut repetition_end = end_length / pattern_size;

        if end_length % pattern_size > 0 { repetition_end += 1; }

        for repetition in repetition_start..=repetition_end {
            if repetition > 1 {
                vec_of_repetitions.push(
                    RepetitionPattern{
                        start: pattern_start,
                        end: pattern_end,
                        repetitions: repetition as i64,
                    }
                )
            }
        }
    }
}

fn create_invalid_id(pattern: i64, repetition: i64) -> i64 {
    let mut str_number = String::new();

    for _ in 0..repetition {
        str_number.push_str(&(pattern.to_string()));
    }
    return str_number.parse::<i64>().expect("Could not parse number");
}

fn get_invalid_ids_in_range(start: i64, end: i64, invalid_ids: &mut Vec<i64>) {
    let mut vec_repetitions: Vec<RepetitionPattern> = Vec::new();

    get_repetition_pattern(start, end, &mut vec_repetitions);

    for repetition_pattern in vec_repetitions {
        for number in repetition_pattern.start..=repetition_pattern.end {
            let invalid_id = create_invalid_id(number, repetition_pattern.repetitions);

            if (start <= invalid_id) && (invalid_id <= end) && (! invalid_ids.contains(&invalid_id)){
                invalid_ids.push(invalid_id);
            }
        }
    }
}