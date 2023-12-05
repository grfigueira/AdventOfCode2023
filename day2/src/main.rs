use std::fs;
use std::str;

fn main() {
    part1();
}

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
                        if(!_valid){
                            break;
                        }
                    }, 
                    "green" => {
                        _valid = _valid && _color_count <= 13;
                        if(!_valid){
                            break;
                        }
                    },
                    "blue" => {
                        _valid = _valid && _color_count <= 14;
                        if(!_valid){
                            break;
                        }
                    },
                    _ => println!("Color is Unknown"),
                }

            }
        }

        if(_valid){
            println!("Game {} is valid", i+1); 
            res += (i+1).clone() as i32;
        }

    }
    println!("The sum of the valid games is {}", res)
}
