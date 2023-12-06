fn main() {
    let mut scratchcards: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input.txt")
        .expect("Failed to read file")
        .lines()
    {
        scratchcards.push(line.trim().to_string());
    }

    let mut points: i32 = 0;

    for card in scratchcards.iter_mut() {
        *card = card[10..].to_string();
    }

    let mut cards: [i32; 186] = [1; 186];

    for (i, line) in scratchcards.iter().enumerate() {
        let mut line_points: i32 = 1;

        let mut counter: i32 = 0;

        let (winning, mut my): (&str, &str) = line.split_at(line.find('|').unwrap() - 1);

        my = &my[3..];

        let winning_vec: Vec<&str> = winning.split(' ').collect();
        let my_vec: Vec<&str> = my.split(' ').collect();

        for num in winning_vec {
            if my_vec.contains(&num) && num != "" {
                line_points *= 2;
                counter += 1;
            }
        }

        for j in 1..=counter {
            cards[i + j as usize] += cards[i];
        }

        line_points /= 2;

        points += line_points;
    }

    println!("Total points: {}", points);

    let mut total_cards: i32 = 0;

    for card in cards.iter() {
        total_cards += card;
    }

    println!("Total cards: {}", total_cards);
}
