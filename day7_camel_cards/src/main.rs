use std::cmp::Ordering;

fn card_value(card: &str) -> i32 {
    match card {
        "T" => 10,
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        _ => card.parse::<i32>().expect("Invalid card rank"), // This will only work for numeric cards.
    }
}

fn parse_hand(hand: String) -> (String, i32, i32) {
    let extracted_hand: String = String::from(&hand[..5]);
    let bid: i32 = hand[6..].parse::<i32>().unwrap();

    let mut cards: Vec<&str> = extracted_hand.split("").collect();

    cards.sort();

    let mut card_quans: Vec<i32> = Vec::new();
    let mut curr_quan: i32 = 1;

    let mut j_cards: i32 = match cards[0] {
        "J" => 1,
        _ => 0,
    };

    for (i, _) in cards.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if cards[i] == "J" {
            j_cards += 1;
        } else if cards[i] == cards[i - 1] {
            curr_quan += 1;
        } else {
            if curr_quan > 1 {
                card_quans.push(curr_quan);
            }
            curr_quan = 1;
        }
    }

    if card_quans.len() != 0 {
        card_quans.remove(0);
    }

    if curr_quan > 1 {
        card_quans.push(curr_quan);
    }

    if card_quans.len() != 0 {
        card_quans.sort();
        card_quans.reverse();
        card_quans[0] += j_cards;
    }

    let mut value: i32 = 0;

    if card_quans.len() == 0 && j_cards > 0 {
        card_quans.push(j_cards + 1);
    }

    if card_quans.len() == 1 {
        if card_quans[0] == 2 {
            value = 1;
        } else if card_quans[0] == 3 {
            value = 3;
        } else if card_quans[0] == 4 {
            value = 5;
        } else if card_quans[0] == 5 {
            value = 6;
        }
    } else if card_quans.len() == 2 {
        if (card_quans[0] == 2 && card_quans[1] == 3) | (card_quans[0] == 3 && card_quans[1] == 2) {
            value = 4;
        } else {
            value = 2;
        }
    }

    if value == 0 && j_cards > 0 {
        value = 6;
    }

    (extracted_hand, bid, value)
}

fn compare_hands(hand1: &(String, i32, i32), hand2: &(String, i32, i32)) -> Ordering {
    if hand1.2 == hand2.2 {
        for (i, _) in hand1.0.chars().enumerate() {
            if card_value(&hand1.0[i..i + 1]) > card_value(&hand2.0[i..i + 1]) {
                return Ordering::Greater;
            } else if card_value(&hand1.0[i..i + 1]) < card_value(&hand2.0[i..i + 1]) {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    } else if hand1.2 > hand2.2 {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

fn main() {
    let mut data: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
    {
        data.push(line.trim().to_string());
    }

    let mut hands: Vec<(String, i32, i32)> = Vec::new();

    for line in data {
        hands.push(parse_hand(line));
    }

    hands.sort_by(|a, b| compare_hands(a, b));

    let mut total_winnings: i32 = 0;

    for hand in &hands {
        if hand.0.contains("J") {
            println!("{:?}", hand);
        }
    }

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += hand.1 * (i as i32 + 1);
    }

    println!("{}", total_winnings);
}

// 248811015
// 249125835
// 249506014
// 249722254
