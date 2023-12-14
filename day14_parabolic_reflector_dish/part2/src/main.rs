fn move_north(rocks: &Vec<String>) -> Vec<String> {
    let mut rocks_chars: Vec<Vec<char>> = rocks.iter().map(|s| s.chars().collect()).collect();

    for (i, line) in rocks.iter().enumerate() {
        for j in 0..line.len() {
            if rocks_chars[i][j] == '.' {
                for k in i..rocks.len() {
                    if rocks_chars[k][j] == '#' {
                        break;
                    } else if rocks_chars[k][j] == 'O' {
                        rocks_chars[i][j] = 'O';
                        rocks_chars[k][j] = '.';
                        break;
                    }
                }
            }
        }
    }
    rocks_chars.iter().map(|s| s.iter().collect()).collect()
}

fn move_west(rocks: &Vec<String>) -> Vec<String> {
    let mut rocks_chars: Vec<Vec<char>> = rocks.iter().map(|s| s.chars().collect()).collect();

    for (i, line) in rocks.iter().enumerate() {
        for j in 0..line.len() {
            if rocks_chars[i][j] == '.' {
                for k in j..line.len() {
                    if rocks_chars[i][k] == '#' {
                        break;
                    } else if rocks_chars[i][k] == 'O' {
                        rocks_chars[i][j] = 'O';
                        rocks_chars[i][k] = '.';
                        break;
                    }
                }
            }
        }
    }
    rocks_chars.iter().map(|s| s.iter().collect()).collect()
}

fn move_south(rocks: &Vec<String>) -> Vec<String> {
    let mut rocks_chars: Vec<Vec<char>> = rocks.iter().map(|s| s.chars().collect()).collect();

    for (i, line) in rocks.iter().enumerate().rev() {
        for j in 0..line.len() {
            if rocks_chars[i][j] == '.' {
                for k in (0..i).rev() {
                    if rocks_chars[k][j] == '#' {
                        break;
                    } else if rocks_chars[k][j] == 'O' {
                        rocks_chars[i][j] = 'O';
                        rocks_chars[k][j] = '.';
                        break;
                    }
                }
            }
        }
    }
    rocks_chars.iter().map(|s| s.iter().collect()).collect()
}

fn move_east(rocks: &Vec<String>) -> Vec<String> {
    let mut rocks_chars: Vec<Vec<char>> = rocks.iter().map(|s| s.chars().collect()).collect();

    for (i, line) in rocks.iter().enumerate() {
        for j in (0..line.len()).rev() {
            if rocks_chars[i][j] == '.' {
                for k in (0..j).rev() {
                    if rocks_chars[i][k] == '#' {
                        break;
                    } else if rocks_chars[i][k] == 'O' {
                        rocks_chars[i][j] = 'O';
                        rocks_chars[i][k] = '.';
                        break;
                    }
                }
            }
        }
    }
    rocks_chars.iter().map(|s| s.iter().collect()).collect()
}

fn count_total(rocks: &Vec<String>) -> i32 {
    let mut total = 0;
    let mut wager: usize = rocks.len();
    for line in rocks.iter() {
        for c in line.chars() {
            if c == 'O' {
                total += wager as i32;
            }
        }
        wager -= 1;
    }
    total
}

fn main() {
    let mut rocks: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input_test.txt")
        .expect("File not found!")
        .lines()
    {
        rocks.push(line.trim().to_string());
    }

    let mut new_rocks: Vec<String> = move_north(&rocks);
    for _ in 0..1000000000 {
        new_rocks = move_north(&new_rocks);
        new_rocks = move_west(&new_rocks);
        new_rocks = move_south(&new_rocks);
        new_rocks = move_east(&new_rocks);
    }

    for line in new_rocks.iter() {
        println!("{}", line);
    }

    let total: i32 = count_total(&new_rocks);

    println!("Total: {}", total);
}
