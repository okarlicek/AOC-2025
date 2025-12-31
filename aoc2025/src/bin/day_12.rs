use std::fs;

const PATH: &str = "inputs/day_12.txt";

struct Box {
    max_space: u32,
    real_space: u32,
}

struct Region {
    region_size: u32,
    boxes_count: Vec<u32>,
}

struct Solution {
    possible: u32,
    impossible: u32,
    unknown: u32,
}

fn main() {
    let mut boxes: Vec<Box> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    process_file(PATH, &mut boxes, &mut regions);
    let solution_a = heuristic_solution_part_a(&boxes, &regions);

    println!("Solution part A:");
    println!("possible: {}", solution_a.possible);
    println!("inpossible: {}", solution_a.impossible);
    println!("unknown: {}", solution_a.unknown);
}

fn heuristic_solution_part_a(boxes: &Vec<Box>, regions: &Vec<Region>) -> Solution {
    let (mut possible, mut impossible, mut unknown) = (0, 0, 0);

    for region in regions {
        let mut boxes_max_space = 0;
        let mut boxes_true_space = 0;

        for (i, b) in region.boxes_count.iter().enumerate() {
            boxes_max_space += boxes[i].max_space * b;
            boxes_true_space += boxes[i].real_space * b;
        }

        if boxes_max_space <= region.region_size {
            possible += 1;
        } else if boxes_true_space > region.region_size {
            impossible += 1;
        } else {
            unknown += 1;
        }
    }
    Solution{possible, impossible, unknown}
}


fn process_file(path: &str, boxes: &mut Vec<Box>, regions: &mut Vec<Region>) {
    let contents = fs::read_to_string(path).unwrap();

    let mut is_box = false;
    let mut box_size = 0;
    let mut real_box_size = 0;

    for line in contents.lines() {
        if is_box && line == "" {
            is_box = false;
            boxes.push(Box{max_space: box_size, real_space: real_box_size});
            continue
        }

        if is_box {
            for c in line.chars() {
                box_size += 1;
                real_box_size += (c == '#') as u32;
            }
            continue;
        }

        if line.chars().last().unwrap() == ':' {
            box_size = 0;
            real_box_size = 0;
            is_box = true;
            continue;
        }

        process_region(line, regions);
    }
}

fn process_region(line: &str, regions: &mut Vec<Region>) {
    let (region_size, boxes_count_str) = line.split_once(":").unwrap();

    let (w_str, h_str) = region_size.split_once("x").unwrap();
    let w: u32 = w_str.parse().unwrap();
    let h: u32 = h_str.parse().unwrap();

    let mut boxes_count: Vec<u32> = Vec::new();
    for symbol in boxes_count_str.trim().split(' ') {
        boxes_count.push(symbol.parse().unwrap());
    }

    regions.push(Region{ region_size: w*h, boxes_count });
}