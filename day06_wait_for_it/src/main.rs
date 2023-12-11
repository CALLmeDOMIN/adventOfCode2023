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

    let mut new_time_string: String = String::new();
    let mut new_distance_string: String = String::new();

    let mut ways_to_beat_vec: Vec<i32> = Vec::new();

    for i in 0..time.len() {
        let mut ways_to_beat: i32 = 0;

        let race_time: i32 = time[i].parse::<i32>().unwrap();
        let distance_to_beat: i32 = distance[i].parse::<i32>().unwrap();

        //part2
        new_time_string += time[i];
        new_distance_string += distance[i];

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

    println!("part1: {}", multiply);

    //part 2
    let new_time: u64 = new_time_string.parse::<u64>().unwrap();
    let new_distance: u64 = new_distance_string.parse::<u64>().unwrap();

    let mut new_ways_to_beat: u64 = 0;

    for k in 1..new_time {
        if ((new_time - k) * k) > new_distance {
            new_ways_to_beat += 1;
        }
    }

    println!("part2: {}", new_ways_to_beat);
}
