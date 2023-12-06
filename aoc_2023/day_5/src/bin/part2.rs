use std::{ops::Range, sync::Arc, thread};

// my cpu: 🔥 🔥 🔥 🔥

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> u64 {
    let mut seed_to_soil: Vec<(Range<u64>, u64)> = vec![];
    let mut soil_to_fertilizer: Vec<(Range<u64>, u64)> = vec![];
    let mut fertilizer_to_water: Vec<(Range<u64>, u64)> = vec![];
    let mut water_to_light: Vec<(Range<u64>, u64)> = vec![];
    let mut light_to_temperature: Vec<(Range<u64>, u64)> = vec![];
    let mut temperature_to_humidity: Vec<(Range<u64>, u64)> = vec![];
    let mut humidity_to_location: Vec<(Range<u64>, u64)> = vec![];

    let seeds_line = input.lines().next().unwrap();

    input.split("\n\n").skip(1).for_each(|map_str| {
        let mut lines = map_str.lines();

        let (map_name, _) = lines.next().unwrap().split_once(" ").unwrap();

        let map = match map_name {
            "seed-to-soil" => &mut seed_to_soil,
            "soil-to-fertilizer" => &mut soil_to_fertilizer,
            "fertilizer-to-water" => &mut fertilizer_to_water,
            "water-to-light" => &mut water_to_light,
            "light-to-temperature" => &mut light_to_temperature,
            "temperature-to-humidity" => &mut temperature_to_humidity,
            "humidity-to-location" => &mut humidity_to_location,
            _ => unreachable!(),
        };

        lines.for_each(|line| {
            let mut segments = line.split_whitespace();
            let dest_start = segments.next().unwrap().parse::<u64>().unwrap();
            let source_start = segments.next().unwrap().parse::<u64>().unwrap();
            let length = segments.next().unwrap().parse::<u64>().unwrap();

            map.push((source_start..source_start + length, dest_start));
        });
    });

    let seed_to_soil: Arc<Vec<(Range<u64>, u64)>> = Arc::new(seed_to_soil);
    let soil_to_fertilizer: Arc<Vec<(Range<u64>, u64)>> = Arc::new(soil_to_fertilizer);
    let fertilizer_to_water: Arc<Vec<(Range<u64>, u64)>> = Arc::new(fertilizer_to_water);
    let water_to_light: Arc<Vec<(Range<u64>, u64)>> = Arc::new(water_to_light);
    let light_to_temperature: Arc<Vec<(Range<u64>, u64)>> = Arc::new(light_to_temperature);
    let temperature_to_humidity: Arc<Vec<(Range<u64>, u64)>> = Arc::new(temperature_to_humidity);
    let humidity_to_location: Arc<Vec<(Range<u64>, u64)>> = Arc::new(humidity_to_location);

    let tasks = seeds_line[7..]
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .flat_map(|chunk| {
            let start = chunk[0].parse::<u64>().unwrap();
            let len = chunk[1].parse::<u64>().unwrap();
            let range = start..start + len;

            range
                .step_by(10000000)
                .map(move |start| start..std::cmp::min(start + 10000000, start + len))
        })
        .enumerate()
        .map(|(i, range)| {
            let seed_to_soil = seed_to_soil.clone();
            let soil_to_fertilizer = soil_to_fertilizer.clone();
            let fertilizer_to_water = fertilizer_to_water.clone();
            let water_to_light = water_to_light.clone();
            let light_to_temperature = light_to_temperature.clone();
            let temperature_to_humidity = temperature_to_humidity.clone();
            let humidity_to_location = humidity_to_location.clone();

            thread::spawn(move || {
                let result = range
                    .clone()
                    .map(|seed| {
                        if seed % 1000000 == 0 {
                            let n = seed - range.start;
                            println!(
                                "task {}: {:.2}% ({}/{})",
                                i,
                                (n as f64 / (range.end - range.start) as f64) * 100.0,
                                n,
                                range.end - range.start
                            )
                        }

                        let soil = get_mapping(&seed_to_soil, seed);
                        let fertilizer = get_mapping(&soil_to_fertilizer, soil);
                        let water = get_mapping(&fertilizer_to_water, fertilizer);
                        let light = get_mapping(&water_to_light, water);
                        let temperature = get_mapping(&light_to_temperature, light);
                        let humidity = get_mapping(&temperature_to_humidity, temperature);
                        get_mapping(&humidity_to_location, humidity)
                    })
                    .min()
                    .unwrap();

                println!("task {}: is done, result: {}", i, result);

                result
            })
        })
        .collect::<Vec<_>>();

    tasks
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .min()
        .unwrap()
}

fn get_mapping(map: &Vec<(Range<u64>, u64)>, val: u64) -> u64 {
    map.iter()
        .find_map(|(range, dest_start)| {
            if range.contains(&val) {
                Some(val - range.start + dest_start)
            } else {
                None
            }
        })
        .unwrap_or(val)
}
