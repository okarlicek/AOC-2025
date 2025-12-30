use std::fs;
use disjoint::DisjointSet;

const PATH: &str = "inputs/day_08.txt";
const NUMBER_OF_JOINS: usize = 1_000;

struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

struct BoxesDistance {
    left_index: usize,
    right_index: usize,
    distance: i64,
}

fn main() {
    let junction_boxes: &mut Vec<JunctionBox> = &mut Vec::new();
    let boxes_distances: &mut Vec<BoxesDistance> = &mut Vec::new();

    process_file(junction_boxes, PATH);
    calculate_distances(junction_boxes, boxes_distances);
    sort_distances(boxes_distances);

    let disjoint_set: &mut DisjointSet = &mut DisjointSet::with_len(junction_boxes.len());
    join_boxes_answer_a(boxes_distances, disjoint_set, NUMBER_OF_JOINS);

    let answer_a = get_answer_a(disjoint_set);
    let answer_b = join_boxes_and_get_answer_b(junction_boxes, boxes_distances, disjoint_set, NUMBER_OF_JOINS);

    println!("Answer A: {}", answer_a);
    println!("Answer B: {}", answer_b);
}

fn get_answer_a(disjoint_set: &DisjointSet) -> usize {
    let mut answer_a = 1;
    let mut sets= disjoint_set.sets();

    sets.sort_by(|a, b| b.len().cmp(&a.len()));

    for set in sets[0..3].iter() {
        println!("{:?}", set);
        answer_a *= set.len();
    }

    answer_a
}

fn join_boxes_answer_a(sorted_distances: &Vec<BoxesDistance>, disjoint_set: &mut DisjointSet, n_joins: usize) {
    for box_distance in sorted_distances[0..n_joins].iter() {
        disjoint_set.join(box_distance.left_index, box_distance.right_index);
    }
}

fn join_boxes_and_get_answer_b(
    junction_boxes: &Vec<JunctionBox>, sorted_distances: &Vec<BoxesDistance>, disjoint_set: &mut DisjointSet, n_joins: usize
) -> i64 {
    for box_distance in sorted_distances[n_joins..].iter() {
        disjoint_set.join(box_distance.left_index, box_distance.right_index);

        if disjoint_set.sets().len() == 1 {
            return junction_boxes[box_distance.left_index].x * junction_boxes[box_distance.right_index].x;
        }
    }
    return 0;
}

fn sort_distances(distances: &mut Vec<BoxesDistance>) {
    distances.sort_by(|a, b| a.distance.cmp(&b.distance));
}

fn calculate_distances(junction_boxes: &mut Vec<JunctionBox>, boxes_distances: &mut Vec<BoxesDistance>) {
    for left_index in 0..junction_boxes.len() {
        for right_index in left_index + 1..junction_boxes.len() {
            boxes_distances.push(
                BoxesDistance{
                    left_index,
                    right_index,
                    distance: euclidean_distance(&junction_boxes[left_index], &junction_boxes[right_index]) }
            )
        }
    }
}

fn euclidean_distance(left_box: &JunctionBox, right_box: &JunctionBox) -> i64 {
    let dx = left_box.x - right_box.x;
    let dy = right_box.y - left_box.y;
    let dz = right_box.z - left_box.z;

    dx * dx + dy * dy + dz * dz
}

fn process_file(junction_boxes: &mut Vec<JunctionBox>, path: &str) {
    let contents = fs::read_to_string(path).unwrap();

    for line in contents.lines() {
        let coords: Vec<i64> = line
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if let [x, y, z] = coords[..] {
            junction_boxes.push(JunctionBox { x, y, z });
        }
    }
}