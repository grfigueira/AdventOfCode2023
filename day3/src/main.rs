use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    part1();
    println!("\n-- Execution time = {:?} --", start.elapsed());
}

#[allow(dead_code)]
fn part1() {
    let input_matrix: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("Should have read the input")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut _res: i32 = 0;

    for (i, line) in input_matrix.iter().enumerate() {
        let mut _num = Vec::new();

        for (j, c) in line.iter().enumerate() {
            if c.is_numeric() && j < line.len() - 1 {
                _num.push(j as i16);
            } else if !_num.is_empty() || (c.is_numeric() && j == line.len() - 1) {
                if c.is_numeric() {
                    _num.push(j as i16);
                }

                if is_part_number(_num.clone(), i as i16, input_matrix.clone()) {
                    let mut _num_str = String::new();

                    for x in _num.clone() {
                        _num_str.push(input_matrix[i][x as usize]);
                    }

                    let _actual_num = _num_str.parse::<i32>().unwrap();
                    println!("Found part number: {}", _actual_num);
                    _res += _actual_num;
                }

                _num.clear();
            }
        }
    }

    println!("Result = {}", _res);
}

fn is_part_number(mut _num: Vec<i16>, _y_coord: i16, matrix: Vec<Vec<char>>) -> bool {

    let mut _res: bool = false;

    for x in _num[0] - 1.._num.last().unwrap() + 2 {
        for y in _y_coord - 1.._y_coord + 2 {

            if x < 0 || y < 0 || y >= matrix.len() as i16 || x >= matrix[y as usize].len() as i16 {
                continue; // Out of bounds
            }

            if !matrix[y as usize][x as usize].is_numeric()
                && matrix[y as usize][x as usize] != '.'{
                    return true;
                }
        }
    }
    return false;
}

/////////////////////////////////////////////
////////////// Part 2 ///////////////////////
/////////////////////////////////////////////

#[allow(dead_code)]
fn part2() {
    let input_matrix: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("Should have read the input")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut _res: i32 = 0;

    // Vector that a triple of (x, y, Vec<i32>) where Vec<i32> is the part numbers
    // and x, y are the coordinates of the '*'character
    let mut _gears: HashMap<(i16, i16), Vec<i32>> = HashMap::new();

    for (i, line) in input_matrix.iter().enumerate() {
        let mut _num_x_coords = Vec::new();

        for (j, c) in line.iter().enumerate() {
            if c.is_numeric() && j < line.len() - 1 {
                _num_x_coords.push(j as i16);
            } else if !_num_x_coords.is_empty() || (c.is_numeric() && j == line.len() - 1) {
                if c.is_numeric() {
                    _num_x_coords.push(j as i16);
                }
                is_gear(
                    _num_x_coords.clone(),
                    i as i16,
                    input_matrix.clone(),
                    &mut _gears,
                );
                _num_x_coords.clear();
            }
        }
    }

    for (_key, value) in _gears.iter() {
        if value.len() == 2 {
            println!("Found gear: {:?}", value);
            _res += value[0] * value[1];
        }
    }

    println!("Result = {}", _res);
}

fn is_gear(
    mut _num: Vec<i16>,
    y_coord: i16,
    matrix: Vec<Vec<char>>,
    _gears: &mut HashMap<(i16, i16), Vec<i32>>,
) -> bool {
    for x in _num[0] - 1.._num.last().unwrap() + 2 {
        for y in y_coord - 1..y_coord + 2 {

            if x < 0 || y < 0 || y >= matrix.len() as i16 || x >= matrix[y as usize].len() as i16 {
                continue; // Out of bounds
            }

            if matrix[y as usize][x as usize] == '*' {
                if !_gears.contains_key(&(x, y)) {
                    _gears.insert((x, y), Vec::new());
                }

                let mut _num_str = String::new();

                for x_coord in _num.clone() {
                    _num_str.push(matrix[y_coord as usize][x_coord as usize]);
                }

                let _actual_num = _num_str.parse::<i32>().unwrap();

                if _gears.get(&(x, y)).unwrap().len() < 2 {
                    _gears.get_mut(&(x, y)).unwrap().push(_actual_num);
                }
                return true;
            }
        }
    }

    return false;
}
