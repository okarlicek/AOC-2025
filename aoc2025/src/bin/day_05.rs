use std::cmp::max;
use std::fs;

const PATH : &str = "./inputs/day_05.txt";

struct IngredientRange {
    start: u64,
    end: u64,
}

fn main() {
    let mut ingredients: &mut Vec<u64> = &mut Vec::new();
    let mut ranges: &mut Vec<IngredientRange> = &mut Vec::new();

    process_file(PATH, &mut ingredients, &mut ranges);
    ingredients.sort();
    ranges.sort_by(|a, b| (a.start, a.end).cmp(&(b.start, b.end)));

    let answer_a = get_answer_a(ingredients, ranges);
    let answer_b = get_answer_b(ranges);

    println!("Answer A: {}", answer_a);
    println!("Answer B: {}", answer_b);
}

fn process_file(
    path: &str, ingredients: &mut Vec<u64>, ranges: &mut Vec<IngredientRange>
) {
    let contents = fs::read_to_string(path).unwrap();
    let mut ranges_flag = true;

    for line in contents.lines() {
        if line == "" {
            ranges_flag = false;
        } else if ranges_flag {
            let (start, end) = line.split_once("-").unwrap();
            ranges.push(IngredientRange {
                start: start.parse::<u64>().unwrap(),
                end: end.parse::<u64>().unwrap(),
            })
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }
}


fn get_answer_a(ingredients: &Vec<u64>, ranges: &Vec<IngredientRange>) -> u64 {
    let mut it_ranges = ranges.iter().peekable();

    let mut answer_a = 0;
    for &ingredient in ingredients.iter() {
        while let Some(range) = it_ranges.peek() {
            // println!("Comparing ingredient {}, with {}-{}", ingredient, range.start, range.end);
            if ingredient >= range.start && ingredient <= range.end {
                answer_a += 1;
                break;
            }

            if ingredient < range.start {
                break;
            }

            it_ranges.next();
        }
    }
    return answer_a;
}

fn get_answer_b(ranges: &Vec<IngredientRange>) -> u64 {
    let mut previous_end: u64 = 0;
    let mut answer_b: u64 = 0;

    for range in ranges {
        if previous_end >= range.start {
            answer_b += range.end.saturating_sub(previous_end);
        } else {
            answer_b += range.end.saturating_sub(range.start) + 1
        }
        previous_end = max(range.end, previous_end);
    }

    return answer_b;
}