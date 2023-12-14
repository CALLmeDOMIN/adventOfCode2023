fn move_north(rocks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rocks_chars: Vec<Vec<char>> = rocks.clone();

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
    rocks_chars
}

fn move_west(rocks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rocks_chars: Vec<Vec<char>> = rocks.clone();

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
    rocks_chars
}

fn move_south(rocks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rocks_chars: Vec<Vec<char>> = rocks.clone();

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
    rocks_chars
}

fn move_east(rocks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rocks_chars: Vec<Vec<char>> = rocks.clone();

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
    rocks_chars
}

fn count_total(map: &Vec<Vec<char>>) -> usize {
    let mut total_load = 0;
    for (vertical_pos, line) in map.iter().enumerate() {
        for (_, c) in line.iter().enumerate() {
            if *c == 'O' {
                total_load += map.len() - vertical_pos;
            }
        }
    }

    total_load
}

fn cycle(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let map = move_north(map);
    let map = move_west(map);
    let map = move_south(map);
    move_east(map)
}

fn main() {
    let mut rocks: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
    {
        rocks.push(line.trim().to_string());
    }

    let mut map: Vec<Vec<char>> = rocks.iter().map(|line| line.chars().collect()).collect();

    let mut seen_states: Vec<Vec<Vec<char>>> = vec![map.clone()];

    loop {
        map = cycle(map.clone());
        if let Some(index) = seen_states.iter().position(|x| x == &map) {
            let cycle_length = seen_states.len() - index;
            let cycle_start = index;
            let final_map =
                seen_states[cycle_start + (1000000000 - cycle_start) % cycle_length].clone();

            let load = count_total(&final_map);
            println!("Load: {}", load);
            return;
        }
        seen_states.push(map.clone());
    }
}
