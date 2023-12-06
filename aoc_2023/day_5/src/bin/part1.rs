use std::ops::Range;

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> u64 {
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

    seeds_line[7..]
        .split_whitespace()
        .map(|seed| {
            let seed = seed.parse::<u64>().unwrap();
            let soil = get_mapping(&seed_to_soil, seed);
            let fertilizer = get_mapping(&soil_to_fertilizer, soil);
            let water = get_mapping(&fertilizer_to_water, fertilizer);
            let light = get_mapping(&water_to_light, water);
            let temperature = get_mapping(&light_to_temperature, light);
            let humidity = get_mapping(&temperature_to_humidity, temperature);
            get_mapping(&humidity_to_location, humidity)
        })
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
