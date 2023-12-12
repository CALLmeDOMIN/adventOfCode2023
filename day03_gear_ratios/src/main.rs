use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let contents_vec = contents.split('\n').collect::<Vec<&str>>();

    let mut sum = 0;

    const SYMBOLS: [char; 10] = ['/', '*', '+', '=', '@', '#', '$', '%', '-', '&'];

    let mut numbers: Vec<(String, bool)> = Vec::<(String, bool)>::new();

    for (i, line) in contents_vec.iter().enumerate() {
        let mut num = String::new();

        let mut symbol: bool = false;

        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                num.push(c);

                if j > 0 {
                    if SYMBOLS.contains(&line.chars().nth(j - 1).unwrap_or(' ')) {
                        symbol = true;
                    }
                }

                if i > 0 {
                    if SYMBOLS.contains(&contents_vec[i - 1].chars().nth(j).unwrap_or(' '))
                        || SYMBOLS.contains(&contents_vec[i - 1].chars().nth(j + 1).unwrap_or(' '))
                    {
                        symbol = true;
                    }

                    if j > 0 {
                        if SYMBOLS.contains(&contents_vec[i - 1].chars().nth(j - 1).unwrap_or(' '))
                        {
                            symbol = true;
                        }
                    }
                }

                if i < contents_vec.len() - 1 {
                    if SYMBOLS.contains(&contents_vec[i + 1].chars().nth(j).unwrap_or(' '))
                        || SYMBOLS.contains(&contents_vec[i + 1].chars().nth(j + 1).unwrap_or(' '))
                    {
                        symbol = true;
                    }

                    if j > 0 {
                        if SYMBOLS.contains(&contents_vec[i + 1].chars().nth(j - 1).unwrap_or(' '))
                        {
                            symbol = true;
                        }
                    }
                }

                if j < line.len() - 1 {
                    if SYMBOLS.contains(&line.chars().nth(j + 1).unwrap_or(' ')) {
                        symbol = true;
                    }
                }

                if j < line.len() - 1 {
                    if line.chars().nth(j + 1).unwrap() == '.'
                        || SYMBOLS.contains(&line.chars().nth(j + 1).unwrap())
                    {
                        numbers.push((num.clone(), symbol));
                        num.clear();
                        symbol = false;
                    }
                } else {
                    numbers.push((num.clone(), symbol));
                    num.clear();
                    symbol = false;
                }
            }
        }
    }

    let mut gears: Vec<((usize, usize), i32)> = Vec::<((usize, usize), i32)>::new();

    for (i, line) in contents_vec.iter().enumerate() {
        let mut num = String::new();

        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                num.push(c);

                let mut is_adjacent: bool = false;
                let mut adjacent_i: usize = usize::MAX;
                let mut adjacent_j: usize = usize::MAX;

                if j > 0 {
                    if line.chars().nth(j - 1).unwrap_or(' ') == '*' {
                        is_adjacent = true;
                        adjacent_i = i;
                        adjacent_j = j - 1;
                    }
                }

                if i > 0 {
                    if contents_vec[i - 1].chars().nth(j).unwrap_or(' ') == '*' {
                        is_adjacent = true;
                        adjacent_i = i - 1;
                        adjacent_j = j;
                    } else if contents_vec[i - 1].chars().nth(j + 1).unwrap_or(' ') == '*' {
                        is_adjacent = true;
                        adjacent_i = i - 1;
                        adjacent_j = j + 1;
                    }

                    if j > 0 {
                        if contents_vec[i - 1].chars().nth(j - 1).unwrap_or(' ') == '*' {
                            is_adjacent = true;
                            adjacent_i = i - 1;
                            adjacent_j = j - 1;
                        }
                    }
                }

                if i < contents_vec.len() - 1 {
                    if contents_vec[i + 1].chars().nth(j).unwrap_or(' ') == '*' {
                        is_adjacent = true;
                        adjacent_i = i + 1;
                        adjacent_j = j;
                    } else if contents_vec[i + 1].chars().nth(j + 1).unwrap_or(' ') == '*' {
                        is_adjacent = true;
                        adjacent_i = i + 1;
                        adjacent_j = j + 1;
                    }

                    if j > 0 {
                        if contents_vec[i + 1].chars().nth(j - 1).unwrap_or(' ') == '*' {
                            is_adjacent = true;
                            adjacent_i = i + 1;
                            adjacent_j = j - 1;
                        }
                    }
                }

                if j < line.len() - 1 {
                    if line.chars().nth(j + 1).unwrap_or(' ') == '*' {
                        is_adjacent = true;
                        adjacent_i = i;
                        adjacent_j = j + 1;
                    }
                }

                if line.chars().nth(j + 1).unwrap_or(' ') == '.'
                    || SYMBOLS.contains(&line.chars().nth(j + 1).unwrap_or(' '))
                    || j == line.len() - 1
                {
                    if is_adjacent && adjacent_i != usize::MAX && adjacent_j != usize::MAX {
                        gears.push(((adjacent_i, adjacent_j), num.parse::<i32>().unwrap()));
                    }
                    num.clear();
                }
            }
        }
    }

    gears.sort_by(|a, b| a.0.cmp(&b.0));

    let mut gears_power_sum: i32 = 0;
    let mut curr_sum: i32 = gears[0].1;

    let mut adjacent_numbers: i32 = 1;

    for i in 1..gears.len() {
        if gears[i].0 == gears[i - 1].0 {
            curr_sum += gears[i].1;
            adjacent_numbers += 1;
        } else if adjacent_numbers > 1 {
            gears_power_sum += curr_sum;
            curr_sum = gears[i].1;
        }
    }

    for (num, symbol) in numbers {
        if symbol {
            sum += num.parse::<i32>().unwrap();
        }
    }

    println!("{}", sum);

    println!("{}", gears_power_sum);

    Ok(())
}
