use std::fs;

const PATH: &str = "./inputs/day_01.txt";
const MAXIMUM_NUMBER_POSITION: i32 = 100;
const STARTING_POSITION: i32 = 50;

fn main() {
    let mut position: i32 = STARTING_POSITION;
    let mut previous_position: i32;
    let mut zero_count: i32 = 0;
    let mut zero_clicks_count: i32 = 0;

    let contents = fs::read_to_string(PATH)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        // parse direction and value
        let direction = &line[0..1];
        let value: i32 = line[1..].parse().expect("Could not parse line to i32");

        previous_position = position;

        position = update_position(position, direction, value);
        // check if it clicks on the zero position
        zero_clicks_count += get_clicks_over_zero(position, previous_position);

        // it is circular so we take remainder after division by 100
        position = polish_circular_position(position);
        // check if the position is at 0
        if position == 0 {
            zero_count += 1;
        }
    }

    println!("Problem 1 answer is: {zero_count}");
    println!("Problem 2 answer is: {zero_clicks_count}");
}

fn update_position(starting_position: i32, direction: &str, value: i32) -> i32 {
    // L direction is minus, R direction is plus
    if direction == "L" {
        return starting_position - value;
    } else if direction == "R" {
        return starting_position + value;
    }
    return starting_position
}

fn polish_circular_position(position: i32) -> i32 {
    let remainder = position % MAXIMUM_NUMBER_POSITION;

    if remainder < 0 {
        return MAXIMUM_NUMBER_POSITION + remainder;
    }
    return remainder;
}

fn get_clicks_over_zero(ending_position: i32, starting_position: i32) -> i32 {
    let mut zero_clicks = 0;

    // negative position means we needed to go over the zero
    // also note that the division in rust works differently
    // than the division without remainder in python
    // that is why I need to do this
    if ending_position < 0 && starting_position > 0 {
        zero_clicks += 1;
    }

    if ending_position == 0{
        zero_clicks += 1;
    }

    zero_clicks += (ending_position / MAXIMUM_NUMBER_POSITION).abs();
    return zero_clicks;
}