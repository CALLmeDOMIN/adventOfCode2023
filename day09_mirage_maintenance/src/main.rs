fn main() {
    let mut data: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
    {
        data.push(line.trim().to_string());
    }

    println!("{:?}", data);
}
