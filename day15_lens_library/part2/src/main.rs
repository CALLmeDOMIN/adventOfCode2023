use std::collections::HashMap;

fn hashy_hashy(id: String) -> i32 {
    let mut current_value: i32 = 0;
    let ascii_values: Vec<u8> = id.chars().map(|c| c as u8).collect();
    for val in ascii_values {
        current_value += val as i32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn get_id(txt: String) -> String {
    if txt.ends_with("-") {
        return txt[..txt.len() - 1].to_string();
    }
    txt[..txt.len() - 2].to_string()
}

fn get_val(txt: String) -> String {
    txt[txt.len() - 1..].to_string()
}

fn input_to_boxes(input: Vec<String>) -> HashMap<i32, Vec<String>> {
    let mut boxes: HashMap<i32, Vec<String>> = Default::default();
    for step in input {
        let id: String = get_id(step.clone());
        let val: String = get_val(step.clone());
        let key: i32 = hashy_hashy(id.clone());
        if step.ends_with("-") {
            if let Some(entry) = boxes.get_mut(&key) {
                if entry.len() == 1 && get_id(entry[0].clone()) == id {
                    boxes.remove(&key);
                    continue;
                }
                entry.retain(|e| get_id(e.clone()) != id);
            }
        } else {
            let new_entry: String = id.clone() + " " + val.clone().as_str();

            if let Some(entry) = boxes.get_mut(&key) {
                let mut swapped: bool = false;
                for e in entry.iter_mut() {
                    if get_id(e.clone()) == id {
                        *e = new_entry.clone();
                        swapped = true;
                        break;
                    }
                }
                if !swapped {
                    entry.push(new_entry);
                }
            } else {
                boxes.insert(key, vec![new_entry]);
            }
        }
    }
    boxes
}

fn the_hashy_counter(boxes: HashMap<i32, Vec<String>>) -> i32 {
    let mut sum: i32 = 0;

    for (key, value) in boxes.iter() {
        if value.len() == 0 {
            continue;
        }

        let box_number: i32 = key + 1;
        for (i, val) in value.iter().enumerate() {
            let slot_number: i32 = i as i32 + 1;
            let focal_len: i32 = get_val(val.clone()).parse::<i32>().unwrap();
            sum += box_number * slot_number * focal_len;
        }
    }

    sum
}

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("File not found!");
    let input = file.lines().collect::<Vec<_>>().join(",");

    let boxes: HashMap<i32, Vec<String>> =
        input_to_boxes(input.split(',').map(|s| s.to_string()).collect());

    let total_sum: i32 = the_hashy_counter(boxes.clone());

    println!("{}", total_sum);
}

// 28231 too low
// 228181 too low
// 404846 too high
