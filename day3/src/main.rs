use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    part1();
    println!("\n-- Execution time = {:?} --", start.elapsed());
}

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
    fn is_special_digit(x: i16, y: i16, matrix: Vec<Vec<char>>) -> bool {
        if x < 0 || y < 0 || y >= matrix.len() as i16 || x >= matrix[y as usize].len() as i16 {
            return false;
        }
        return !matrix[y as usize][x as usize].is_numeric()
            && matrix[y as usize][x as usize] != '.';
    }

    let mut _res: bool = false;

    _res = _res
        || is_special_digit(_num[0] - 1, _y_coord, matrix.clone())
        || is_special_digit(_num[0] - 1, _y_coord - 1, matrix.clone())
        || is_special_digit(_num[0] - 1, _y_coord + 1, matrix.clone())
        || is_special_digit(_num.last().unwrap() + 1, _y_coord, matrix.clone())
        || is_special_digit(_num.last().unwrap() + 1, _y_coord - 1, matrix.clone())
        || is_special_digit(_num.last().unwrap() + 1, _y_coord + 1, matrix.clone());

    for x in _num[0] - 1.._num.last().unwrap() + 1 {
        _res = _res
            || is_special_digit(x, _y_coord - 1, matrix.clone())
            || is_special_digit(x, _y_coord + 1, matrix.clone())
    }
    return _res;
}
