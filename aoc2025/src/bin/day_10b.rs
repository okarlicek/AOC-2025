use std::fs;
use good_lp::{variables, variable, Expression, coin_cbc, SolverModel, Solution, constraint};

const PATH: &str = "inputs/day_10.txt";

struct Input {
    lights: Vec<bool>,
    vec_buttons: Vec<Vec<u32>>,
    voltages: Vec<u32>,
}

fn main() {
    let mut vec_inputs: Vec<Input> = Vec::new();

    process_file(PATH, &mut vec_inputs);

    let mut answer_b = 0;
    for input in vec_inputs {
        answer_b += find_solution_lp(&input);
    }


    println!("Answer B: {}", answer_b);
}

fn find_solution_lp(input: &Input) -> u32 {
    let mut vars = variables!();

    let x: Vec<_> = (0..input.vec_buttons.len())
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let objective = x.iter().sum::<Expression>();
    let mut model = vars.minimise(objective).using(coin_cbc);

    // now adding contrainst to match voltage of all machines
    for (i, target_voltage) in input.voltages.iter().enumerate() {
        let mut single_voltage_expression = Expression::from(0.0);

        // iterating through buttons
        for (j, button) in input.vec_buttons.iter().enumerate() {
            if button.contains(&(i as u32)) {
                // if the button "starts" the given machine we add
                // the number of presess to the voltage
                single_voltage_expression += x[j];
            }
        }
        model = model.with(constraint!(single_voltage_expression == *target_voltage))
    }

    let solution = model.solve().expect("no solution found");

    solution.eval(x.iter().sum::<Expression>()).round() as u32
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