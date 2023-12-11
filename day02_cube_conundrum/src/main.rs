use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;

    let content_vec: Vec<&str> = content.split('\n').collect::<Vec<&str>>();

    let content_vec = content_vec[..content_vec.len() - 1].to_vec();

    let mut sum: i32 = 0;

    let mut sum_power: i32 = 0;

    for (i, line) in content_vec.iter().enumerate() {
        let mut id: i32 = 0;
        let mut cubes: Vec<&str> = Vec::new();

        if i < 9 {
            id = line[5..6].parse::<i32>().unwrap();
            cubes = line[7..line.len()].split([',', ';']).collect::<Vec<&str>>();
        } else if i >= 9 && i <= 98 {
            id = line[5..7].parse::<i32>().unwrap();
            cubes = line[8..line.len()].split([',', ';']).collect::<Vec<&str>>();
        } else if i > 98 {
            id = line[5..8].parse::<i32>().unwrap();
            cubes = line[9..line.len()].split([',', ';']).collect::<Vec<&str>>();
        }

        let mut ok: bool = true;

        let mut min_red: i32 = i32::MIN;
        let mut min_green: i32 = i32::MIN;
        let mut min_blue: i32 = i32::MIN;

        for cube in cubes.iter() {
            let cube: String = cube[1..cube.len()].to_owned();
            let (value_str, mut key): (&str, &str) = cube.split_at(cube.find(' ').unwrap());
            key = &key[1..key.len()];
            let value: i32 = value_str.parse::<i32>().unwrap();

            if key == "red" && value <= RED {
                continue;
            } else if key == "green" && value <= GREEN {
                continue;
            } else if key == "blue" && value <= BLUE {
                continue;
            } else {
                ok = false;
                break;
            }
        }

        for cube in cubes.iter() {
            let cube: String = cube[1..cube.len()].to_owned();
            let (value_str, mut key): (&str, &str) = cube.split_at(cube.find(' ').unwrap());
            key = &key[1..key.len()];
            let value: i32 = value_str.parse::<i32>().unwrap();

            if key == "red" && value > min_red {
                min_red = value;
            } else if key == "green" && value > min_green {
                min_green = value;
            } else if key == "blue" && value > min_blue {
                min_blue = value;
            }
        }

        sum_power += min_red * min_green * min_blue;

        if ok {
            sum += id;
        }
    }

    println!("Sum: {}", sum);
    println!("Sum power: {}", sum_power);

    Ok(())
}
