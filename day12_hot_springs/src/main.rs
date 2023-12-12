fn parse_input(data: &Vec<String>) -> Vec<(Vec<String>, &str)> {
    let mut parsed_input: Vec<(Vec<String>, &str)> = Vec::new();

    for line in data {
        let spring_data: Vec<&str> = line.split(" ").collect();
        let mut seq: String = String::new();
        let mut damage: Vec<String> = Vec::new();
        for c in spring_data[0].chars() {
            if c == '?' || c == '#' {
                seq.push(c);
            } else if seq.len() > 0 {
                damage.push(seq.clone());
                seq.clear();
            }
        }

        if seq.len() > 0 {
            damage.push(seq);
        }

        parsed_input.push((damage, spring_data[1]));
    }

    parsed_input
}

fn main() {
    let mut springs: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input_test.txt")
        .expect("File not found!")
        .lines()
    {
        springs.push(line.trim().to_string());
    }

    let parsed_input = parse_input(&springs);

    println!("{:?}", parsed_input);
}
