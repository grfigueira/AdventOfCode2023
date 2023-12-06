use std::fs;
use std::str;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    part2();
    println!("\n-- Execution time = {:?} --", start.elapsed());
}

#[allow(dead_code)]
fn part1()
{
    let input: String = fs::read_to_string("input.txt").expect("Should have read the input");
    let mut res: i32 = 0;

    for(i, line) in input.lines().enumerate(){

        let _game = line.split(":").collect::<Vec<&str>>()[1];
        let mut _valid: bool = true;
        let _sets = _game.split(";").collect::<Vec<&str>>();

        for set in _sets.iter(){

            let _color_set = set.split(",").collect::<Vec<&str>>();

            for color in _color_set.iter(){

                let _color_str = color.split(" ").collect::<Vec<&str>>();
                let _color_count:i32 = _color_str[1].parse::<i32>().unwrap();
                let _color = _color_str[2];

                match _color {
                    "red" => {
                        _valid = _valid && _color_count <= 12;
                        if !_valid {
                            break;
                        }
                    }, 
                    "green" => {
                        _valid = _valid && _color_count <= 13;
                        if !_valid {
                            break;
                        }
                    },
                    "blue" => {
                        _valid = _valid && _color_count <= 14;
                        if !_valid {
                            break;
                        }
                    },
                    _ => println!("Color is Unknown"),
                }

            }
        }

        if _valid {
            println!("Game {} is valid", i+1); 
            res += (i+1).clone() as i32;
        }

    }
    println!("The sum of the valid games is {}", res)
}

#[allow(dead_code)]
fn part2(){
    
    let input: String = fs::read_to_string("input.txt").expect("Should have read the input");
    let mut res: i32 = 0;

    // For each game
    for (i, line) in input.lines().enumerate(){

        let _game = line.split(":").collect::<Vec<&str>>()[1];
        let _sets = _game.split(";").collect::<Vec<&str>>();

        let mut _min_colors = [0, 0, 0];

        //For each set taken from the bag
        for set in _sets.iter(){

            let _color_set = set.split(",").collect::<Vec<&str>>();
            
            // For each colour
            for color in _color_set.iter(){

                let _color_str = color.split(" ").collect::<Vec<&str>>();
                let _color_count:i32 = _color_str[1].parse::<i32>().unwrap();
                let _color = _color_str[2];

                match _color {
                    "red" => {
                        if _color_count >= _min_colors[0]{
                            _min_colors[0] = _color_count;
                        }
                    }, 
                    "green" => {
                        if _color_count >= _min_colors[1]{
                            _min_colors[1] = _color_count;
                        }
                    },
                    "blue" => {
                        if _color_count >= _min_colors[2]{
                            _min_colors[2] = _color_count;
                        }
                    },
                    _ => println!("Color is Unknown"),
                }

            }
        }
        res += _min_colors.iter().fold(1, |acc, &x| acc*x);
        println!("Game {}, has min_colors = {:?}, and the multiplication is {}", i, _min_colors, _min_colors.iter().fold(1, |acc, &x| acc*x));
    }
    println!("\nThe sum of powers {}", res)
}