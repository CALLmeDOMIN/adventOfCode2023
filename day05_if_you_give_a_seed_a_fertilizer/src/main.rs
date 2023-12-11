fn main() {
    let mut almanac: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("input.txt")
        .expect("Failed to read file")
        .lines()
    {
        almanac.push(line.trim().to_string());
    }

    almanac.push(String::from(""));

    let seeds: &Vec<&str> = &almanac[0][7..].split(' ').collect();
    let mut soil: Vec<u64> = vec![0; seeds.len()];
    let mut fertilizer: Vec<u64> = vec![0; seeds.len()];
    let mut water: Vec<u64> = vec![0; seeds.len()];
    let mut light: Vec<u64> = vec![0; seeds.len()];
    let mut temperature: Vec<u64> = vec![0; seeds.len()];
    let mut humidity: Vec<u64> = vec![0; seeds.len()];
    let mut location: Vec<u64> = vec![0; seeds.len()];

    let mut mode: String = String::from("seed");

    for line in almanac[3..].iter() {
        if line.is_empty() {
            for (i, seed) in seeds.iter().enumerate() {
                let seed_num: u64 = seed.parse::<u64>().unwrap();

                match mode.as_str() {
                    "seed" => {
                        if soil[i] == 0 {
                            soil[i] = seed_num
                        }
                    }
                    "soil" => {
                        if fertilizer[i] == 0 {
                            fertilizer[i] = soil[i]
                        }
                    }
                    "fert" => {
                        if water[i] == 0 {
                            water[i] = fertilizer[i]
                        }
                    }
                    "wate" => {
                        if light[i] == 0 {
                            light[i] = water[i]
                        }
                    }
                    "ligh" => {
                        if temperature[i] == 0 {
                            temperature[i] = light[i]
                        }
                    }
                    "temp" => {
                        if humidity[i] == 0 {
                            humidity[i] = temperature[i]
                        }
                    }
                    "humi" => {
                        if location[i] == 0 {
                            location[i] = humidity[i]
                        }
                    }
                    _ => (),
                }
            }
        } else if !line[0..1].chars().all(char::is_numeric) {
            mode = line[..4].to_string();
            continue;
        } else if line[0..1].chars().all(char::is_numeric) {
            let split_line: Vec<&str> = line.split(' ').collect();

            let dst: u64 = split_line[0].parse::<u64>().unwrap();
            let src: u64 = split_line[1].parse::<u64>().unwrap();
            let length: u64 = split_line[2].parse::<u64>().unwrap();

            match mode.as_str() {
                "seed" => {
                    for (i, seed) in seeds.iter().enumerate() {
                        let seed_num: u64 = seed.parse::<u64>().unwrap();

                        if seed_num >= src && seed_num <= src + length - 1 {
                            soil[i] = dst + seed_num - src
                        }
                    }
                }
                "soil" => {
                    for (i, seed) in soil.iter().enumerate() {
                        if *seed >= src && *seed <= src + length - 1 {
                            fertilizer[i] = dst + seed - src
                        }
                    }
                }
                "fert" => {
                    for (i, seed) in fertilizer.iter().enumerate() {
                        if *seed >= src && *seed <= src + length - 1 {
                            water[i] = dst + seed - src
                        }
                    }
                }
                "wate" => {
                    for (i, seed) in water.iter().enumerate() {
                        if *seed >= src && *seed <= src + length - 1 {
                            light[i] = dst + seed - src
                        }
                    }
                }
                "ligh" => {
                    for (i, seed) in light.iter().enumerate() {
                        if *seed >= src && *seed <= src + length - 1 {
                            temperature[i] = dst + seed - src
                        }
                    }
                }
                "temp" => {
                    for (i, seed) in temperature.iter().enumerate() {
                        if *seed >= src && *seed <= src + length - 1 {
                            humidity[i] = dst + seed - src
                        }
                    }
                }
                "humi" => {
                    for (i, seed) in humidity.iter().enumerate() {
                        if *seed >= src && *seed <= src + length - 1 {
                            location[i] = dst + seed - src
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let mut lowest_location: u64 = location[0];

    for loc in location.iter() {
        if *loc < lowest_location {
            lowest_location = *loc;
        }
    }

    println!("Seeds: {:?}", seeds);
    println!("Soil: {:?}", soil);
    println!("Fertilizer: {:?}", fertilizer);
    println!("Water: {:?}", water);
    println!("Light: {:?}", light);
    println!("Temperature: {:?}", temperature);
    println!("Humidity: {:?}", humidity);
    println!("Location: {:?}", location);

    println!("Lowest location: {}", lowest_location);
}

// 214121685 too low
