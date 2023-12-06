fn main() {
    let mut data: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input.txt")
        .expect("Failed to read file")
        .lines()
    {
        data.push(line.trim().to_string());
    }

    let time: Vec<&str> = data[0][13..].split("     ").collect();
    let distance: Vec<&str> = data[1][12..].split("   ").collect();

    let mut ways_to_beat_vec: Vec<i32> = Vec::new();

    for i in 0..time.len() {
        let mut ways_to_beat: i32 = 0;

        let race_time: i32 = time[i].parse::<i32>().unwrap();
        let distance_to_beat: i32 = distance[i].parse::<i32>().unwrap();

        for j in 1..race_time {
            let distance_travelled: i32 = (race_time - j) * j;

            if distance_travelled > distance_to_beat {
                ways_to_beat += 1;
            }
        }
        ways_to_beat_vec.push(ways_to_beat);
    }

    let mut multiply: i32 = 1;

    for integer in ways_to_beat_vec {
        multiply *= integer;
    }

    println!("{:?}", multiply);
}
