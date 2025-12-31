use std::fs;
use std::collections::HashMap;
use std::string::ToString;

const PATH: &str = "inputs/day_11.txt";


fn main() {
    let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut memory: HashMap<String, u64> = HashMap::new();

    process_file(PATH, &mut input_map);

    let answer_a = dfs_solution("you".to_string(), &"out".to_string(), &input_map, &mut memory);
    let answer_b = solution_part_b(&input_map, &mut memory);

    println!("Answer A: {}", answer_a);
    println!("Answer B: {}", answer_b);
}

fn dfs_solution(
    current_point: String, ending_point: &String, input_map: &HashMap<String, Vec<String>>, memory: &mut HashMap<String, u64>,
) -> u64 {

    if memory.contains_key(&current_point) {
        return *memory.get(&current_point).unwrap();
    }

    let mut number_of_paths = 0;

    if current_point == "out" { return number_of_paths; }

    for point in input_map.get(&current_point).unwrap() {
        if point == ending_point {
            number_of_paths += 1;
            continue;
        }
        number_of_paths += dfs_solution(point.to_string(), ending_point, input_map, memory);
    }

    memory.insert(current_point, number_of_paths);
    number_of_paths
}

fn solution_part_b(
    input_map: &HashMap<String, Vec<String>>, memory: &mut HashMap<String, u64>,
) -> u64 {
    // Note that there is no path from dac to fft
    // therefore we can omit path `svr -> dac -> fft -> out`
    // and only calculate `svr -> fft -> dac -> out`
    memory.clear();
    let from_svr_to_fft = dfs_solution("svr".to_string(), &"fft".to_string(), input_map, memory);

    memory.clear();
    let from_fft_to_dac = dfs_solution("fft".to_string(), &"dac".to_string(), input_map, memory);

    memory.clear();
    let from_dac_to_out = dfs_solution("dac".to_string(), &"out".to_string(), input_map, memory);

    from_svr_to_fft * from_fft_to_dac * from_dac_to_out
}

fn process_file(path: &str, input_map: &mut HashMap<String, Vec<String>>) {
    let contents = fs::read_to_string(path).unwrap();

    for line in contents.lines() {
        let (key, values) = line.split_once(":").unwrap();
        let temp_vec_values: Vec<String> = values
            .trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        input_map.insert(
            key.to_string(),
            temp_vec_values
        );
    }
}