fn main() {
    let mut lines: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input_test.txt")
        .expect("File not found!")
        .lines()
    {
        lines.push(line.trim().to_string());
    }

    let mut sum: i32 = 0;
    for line in lines {
        let steps: Vec<&str> = line.split(',').collect();
        for step in steps {
            let mut current_value: i32 = 0;
            let ascii_values: Vec<u8> = step.chars().map(|c| c as u8).collect();
            for val in ascii_values {
                current_value += val as i32;
                current_value *= 17;
                current_value %= 256;
            }
            sum += current_value;
        }
    }
    println!("{}", sum);
}
