use std::fs;
use itertools::Itertools;

const PATH: &str = "inputs/day_10.txt";

struct Input {
    lights: Vec<bool>,
    vec_buttons: Vec<Vec<u32>>,
    voltages: Vec<u32>,
}

fn main() {
    let mut vec_inputs: Vec<Input> = Vec::new();

    process_file(PATH, &mut vec_inputs);

    let mut answer_a = 0;
    for input in vec_inputs {
        answer_a += find_solution_bfs(&input);
    }


    println!("Answer A: {}", answer_a);
}

fn find_solution_bfs(input: &Input) -> u32 {
    for solution in (0..input.vec_buttons.len()).powerset() {
        if try_solution(input, &solution) {
            return solution.len() as u32;
        }
    }
    panic!("No solution found");
}

fn try_solution(input: &Input, solution: &Vec<usize>) -> bool {
    let mut mask: Vec<bool> = vec![false; input.lights.len()];

    for used_button in solution {
        for ligth in &input.vec_buttons[*used_button] {
            mask[*ligth as usize] = !mask[*ligth as usize];
        }
    }

    if are_vectors_same(&mask, &input.lights) {
        return true;
    }

    false
}

fn are_vectors_same(a: &Vec<bool>, b: &Vec<bool>) -> bool {
    let matching = a.iter().zip(b).filter(|&(a, b)| a == b).count();
    matching == a.len()
}

fn process_file(path: &str, vec_inputs: &mut Vec<Input>) {
    let contents = fs::read_to_string(path).unwrap();

    for line in contents.lines() {
        vec_inputs.push(process_line(line));
    }
}

fn process_line(line: &str) -> Input {
    let mut lights: Vec<bool> = Vec::new();
    let mut vec_buttons: Vec<Vec<u32>> = Vec::new();
    let mut voltages: Vec<u32> = Vec::new();

    for part in line.split(' ') {
        let symbol = part.chars().nth(0).unwrap();
        let input_str = part[1..part.len() - 1].to_string();

        match symbol {
            '[' => process_lights(&input_str, &mut lights),
            '(' => process_buttons(&input_str, &mut vec_buttons),
            '{' => process_voltages(&input_str, &mut voltages),
            _ => panic!("Unknown symbol {}", symbol),
        }
    }
    Input{lights, vec_buttons, voltages}
}

fn process_lights(input: &str, lights: &mut Vec<bool>) {
    for c in input.chars() {
        match c {
            '#' => lights.push(true),
            '.' => lights.push(false),
            _ => panic!("Unexpected character: {}", c),
        }
    }
}

fn process_buttons(input: &str, vec_buttons: &mut Vec<Vec<u32>>) {
    let buttons: Vec<u32> = input.split(',').map(|part| part.parse::<u32>().unwrap()).collect();
    vec_buttons.push(buttons);
}

fn process_voltages(input: &str, voltages: &mut Vec<u32>) {
    for volt in input.split(',') {
        voltages.push(volt.parse().unwrap());
    }
}