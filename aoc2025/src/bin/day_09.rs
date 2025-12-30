use std::{fs, i64};
use nalgebra::DMatrix;

const PATH: &str = "inputs/day_09.txt";

struct Point {
    x: i64,
    y: i64,
}

struct TilesAreas {
    xs: Vec<i64>,
    ys: Vec<i64>,
    tiles_sum: DMatrix<i64>,
}

fn main() {
    let vec_points: &mut Vec<Point> = &mut Vec::new();
    process_file(vec_points, PATH);

    let answer_a = get_highest_area_part_a(vec_points);

    let tiles_areas: &TilesAreas = &create_tiles_area(vec_points);
    let answer_b = get_highest_area_part_b(vec_points, tiles_areas);

    println!("Answer A: {}", answer_a);
    println!("Answer B: {}", answer_b);
}

fn get_highest_area_part_a(vec_points: &Vec<Point>) -> i64 {
    let mut max_area: i64 = 0;
    let mut temp_area: i64;

    for i in 0..vec_points.len() {
        for j in i + 1..vec_points.len() {
            temp_area = calculate_area(&vec_points[i], &vec_points[j]);

            max_area = max_area.max(temp_area);
        }
    }
    max_area
}

fn get_highest_area_part_b(vec_points: &Vec<Point>, tiles_areas: &TilesAreas) -> i64 {
    let mut max_area: i64 = 0;
    let mut temp_area: i64;

    for i in 0..vec_points.len() {
        for j in i + 1..vec_points.len() {
            temp_area = calculate_area(&vec_points[i], &vec_points[j]);

            if is_rectangle_inside_polygon(&vec_points[i], &vec_points[j], tiles_areas) {
                max_area = max_area.max(temp_area);
            }
        }
    }
    max_area
}

fn calculate_area(point1: &Point, point2: &Point) -> i64 {
    let dx = (point1.x - point2.x).abs() + 1;
    let dy = (point1.y - point2.y).abs() + 1;

    dx * dy
}

fn is_rectangle_inside_polygon(point1: &Point, point2: &Point, tiles_areas: &TilesAreas) -> bool {
    let x1 = tiles_areas.xs.binary_search(&point1.x.min(point2.x)).unwrap();
    let x2 = tiles_areas.xs.binary_search(&point1.x.max(point2.x)).unwrap();
    let y1 = tiles_areas.ys.binary_search(&point1.y.min(point2.y)).unwrap();
    let y2 = tiles_areas.ys.binary_search(&point1.y.max(point2.y)).unwrap();

    let dx = x2 - x1 + 1;
    let dy = y2 - y1 + 1;

    let potential_area = (dx * dy) as i64;

    let mut tile_area = tiles_areas.tiles_sum[(x2, y2)];
    if x1 > 0 {
        tile_area -= tiles_areas.tiles_sum[(x1.saturating_sub(1), y2)];
    }
    if y1 > 0 {
        tile_area -= tiles_areas.tiles_sum[(x2, y1.saturating_sub(1))];
    }
    if x1 > 0 && y1 > 0 {
        tile_area += tiles_areas.tiles_sum[(x1.saturating_sub(1), y1.saturating_sub(1))];
    }

    tile_area == potential_area
}

fn create_tiles_area(points: &Vec<Point>) -> TilesAreas {
    let mut xs = points
        .iter()
        .map(|p| p.x)
        .collect::<Vec<_>>();
    xs.sort();
    xs.dedup();


    let mut ys = points
        .iter()
        .map(|p| p.y)
        .collect::<Vec<_>>();
    ys.sort();
    ys.dedup();

    let mut tiles_sum: DMatrix<i64> = DMatrix::zeros(xs.len(), ys.len());

    // fill in polygon boundary
    fill_in_boundary(&xs, &ys, points, &mut tiles_sum);
    // fill inside of polygon,
    fill_in_inside_of_polygon(&mut tiles_sum);
    // calculate the left top tiles
    calculate_tiles_left_above(&mut tiles_sum);

    TilesAreas{ xs, ys, tiles_sum }
}

fn fill_in_boundary(xs: &Vec<i64>, ys: &Vec<i64>, points: &Vec<Point>, tiles_sum: &mut DMatrix<i64>) {
    let n = points.len();

    for i in 0..n {
        let point = &points[i];
        let (x_index, y_index) = get_point_indexes(point, xs, ys);

        let next_point = &points[(i + 1) % n];
        let (next_x_index, next_y_index) = get_point_indexes(next_point, xs, ys);

        if x_index == next_x_index {
            let mut y = y_index.min(next_y_index);

            while y <= y_index.max(next_y_index) {
                tiles_sum[(x_index, y)] = 1;
                y += 1;
            }
        }

        if y_index == next_y_index {
            let mut x = x_index.min(next_x_index);

            while x <= x_index.max(next_x_index) {
                tiles_sum[(x, y_index)] = 1;
                x += 1;
            }
        }
    }
}

fn fill_in_inside_of_polygon(tiles_sum: &mut DMatrix<i64>) {
    let (n_rows, n_cols) = tiles_sum.shape();

    for row in 0..n_rows {
        for col in 0..n_cols {
            if tiles_sum[(row, col)] == 1 { continue; }

            // above
            let mut above = false;
            if row > 0 && tiles_sum.column(col).rows(0, row).sum() > 0 {
                above = true;
            }
            // under
            let mut under = false;
            if row + 1 < n_rows && tiles_sum.column(col).rows(row + 1, n_rows - row - 1).sum() > 0 {
                under = true;
            }
            // left
            let mut left = false;
            if col > 0 && tiles_sum.row(row).columns(0, col).sum() > 0 {
                left = true;
            }
            // right
            let mut right = false;
            if col + 1 < n_cols && tiles_sum.row(row).columns(col + 1, n_cols - col - 1).sum() > 0 {
                right = true;
            }

            if above && under && left && right {
                tiles_sum[(row, col)] = 1;
            }
        }
    }
}

fn calculate_tiles_left_above(tiles_sum: &mut DMatrix<i64>) {
    let (n_rows, n_cols) = tiles_sum.shape();

    for row in 0..n_rows {
        for col in 0..n_cols {
            let mut temp_sum = 0;

            // above
            if row > 0 {
                temp_sum += tiles_sum[(row.saturating_sub(1), col)]
            }
            // left
            if col > 0 {
                temp_sum += tiles_sum[(row, col.saturating_sub(1))]
            }
            // substract above left
            if row > 0 && col > 0 {
                temp_sum -= tiles_sum[(row.saturating_sub(1), col.saturating_sub(1))]
            }
            tiles_sum[(row, col)] += temp_sum;
        }
    }
}

fn get_point_indexes(point: &Point, xs: &Vec<i64>, ys: &Vec<i64>) -> (usize, usize) {
    let x_index = xs.binary_search(&point.x).unwrap();
    let y_index = ys.binary_search(&point.y).unwrap();
    (x_index, y_index)
}

fn process_file(vec_points: &mut Vec<Point>, path: &str) {
    let contents = fs::read_to_string(path).unwrap();

    for line in contents.lines() {
        let coords: Vec<i64> = line
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if let [x, y] = coords[..] {
            vec_points.push(Point { x, y });
        }
    }
}