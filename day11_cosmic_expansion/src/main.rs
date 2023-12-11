fn expand_the_universe(data: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut empty_columns: Vec<i32> = Vec::new();
    let mut empty_rows: Vec<i32> = Vec::new();

    for (i, line) in data.iter().enumerate() {
        let mut is_empty: bool = true;
        for c in line.chars() {
            if c == '#' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_rows.push(i as i32);
        }
    }

    for i in 0..data[0].len() {
        let mut is_empty: bool = true;
        for line in data.iter() {
            if line.chars().nth(i).unwrap() == '#' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_columns.push(i as i32);
        }
    }

    return (empty_columns, empty_rows);
}

fn find_cords(data: &Vec<String>) -> Vec<(i32, i32)> {
    let mut cords: Vec<(i32, i32)> = Vec::new();

    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                cords.push((i as i32, j as i32));
            }
        }
    }

    return cords;
}

fn main() {
    let mut data: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
    {
        data.push(line.trim().to_string());
    }

    let (empty_columns, empty_rows) = expand_the_universe(&data);

    let cords: Vec<(i32, i32)> = find_cords(&data);

    let mut distance: u64 = 0;
    for i in 0..cords.len() {
        for j in i + 1..cords.len() {
            let (x1, y1) = cords[i];
            let (x2, y2) = cords[j];

            if x1 < x2 {
                for row in empty_rows.iter() {
                    if *row > x1 && *row < x2 {
                        distance += 999999; //adjust for part 2 - part 1: 1, part 2: 999999
                    }
                }
            } else {
                for row in empty_rows.iter() {
                    if *row < x1 && *row > x2 {
                        distance += 999999; //adjust for part 2 - part 1: 1, part 2: 999999
                    }
                }
            }

            if y1 < y2 {
                for column in empty_columns.iter() {
                    if *column > y1 && *column < y2 {
                        distance += 999999; //adjust for part 2 - part 1: 1, part 2: 999999
                    }
                }
            } else {
                for column in empty_columns.iter() {
                    if *column < y1 && *column > y2 {
                        distance += 999999; //adjust for part 2 - part 1: 1, part 2: 999999
                    }
                }
            }

            let dx: i32 = (x2 - x1).abs();
            let dy: i32 = (y2 - y1).abs();

            distance += (dx + dy) as u64;
        }
    }

    println!("{:?}", cords);
    println!("{:?}", empty_rows);
    println!("{:?}", empty_columns);
    println!("{}", distance);
}
