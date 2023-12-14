fn push_lever(rocks: &Vec<String>) -> Vec<String> {
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
    println!();
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

    for line in std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
    {
        rocks.push(line.trim().to_string());
    }

    let new_rocks = push_lever(&rocks);

    for (i, line) in rocks.iter().enumerate() {
        println!("{} {}", line, new_rocks[i]);
    }

    let total = count_total(&new_rocks);
    println!("Total: {}", total);
}
