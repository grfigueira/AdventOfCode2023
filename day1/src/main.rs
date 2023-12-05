use std::collections::HashMap;
use std::fs;

const DIGITS: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
const DIGITS_REV: [(&str, i32); 9] = [
    ("eno", 1),
    ("owt", 2),
    ("eerht", 3),
    ("ruof", 4),
    ("evif", 5),
    ("xis", 6),
    ("neves", 7),
    ("thgie", 8),
    ("enin", 9),
];

fn main() {
    part2();
}

fn part1() {
    let mut _res: i32 = 0;
    let input: String = fs::read_to_string("input.txt").expect("Should have read the input");

    for (i, line) in input.lines().enumerate() {
        let mut _num = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                _num.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                _num.push(c);
                break;
            }
        }
        let callibarion_value = _num.parse::<i32>().unwrap();
        println!("Callibartion {} is {}", i, callibarion_value);

        _res += callibarion_value;
    }

    println!("The sum of the calibration values is: {}", _res);
}

fn part2() {
    let input: String = fs::read_to_string("input.txt").expect("Should have read the input");

    let mut _res = 0;
    let mut _current_num = String::new();

    for (i, _line) in input.lines().enumerate() {
        _current_num
            .push(std::char::from_digit(get_first_digit(_line.to_string(), false), 10).unwrap());
        _current_num
            .push(std::char::from_digit(get_first_digit(_line.to_string(), true), 10).unwrap());

        let callibration_num = _current_num.parse::<i32>().unwrap();
        println!("Callibartion {} is {}", i + 1, callibration_num);
        _res += callibration_num;
        _current_num = String::new();
    }
    println!("The sum of the calibration values is: {}", _res);
}

fn get_first_digit(line: String, rev: bool) -> u32 {
    let mut _current_first: (usize, u32) = (line.len() - 1, 0);
    let mut _line = String::new();
    let mut _digits: HashMap<&str, i32> = HashMap::new();

    if rev {
        _line = line.chars().rev().collect();
        _digits = DIGITS_REV.iter().cloned().collect();
    } else {
        _line = line.to_string();
        _digits = DIGITS.iter().cloned().collect();
    }

    for d in _digits.keys() {
        let index = match _line.find(d) {
            Some(index) => index,
            None => std::usize::MAX,
        };
        if _current_first.0 >= index {
            _current_first = (index, (*_digits.get(d).unwrap()).try_into().unwrap())
        }
    }

    for (i, c) in _line.chars().enumerate() {
        if c.is_numeric() {
            if i <= _current_first.0 {
                _current_first = (i, c.to_digit(10).unwrap());
            }
        }
    }

    return _current_first.1;
}
