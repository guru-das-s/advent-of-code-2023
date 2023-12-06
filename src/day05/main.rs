use regex::Regex;

const TEST_INPUT: &str = include_str!("testinput");
const INPUT: &str = include_str!("input");

fn get_numvec_from_string(input: &str) -> Vec<u64> {
    let nums_re = Regex::new(r"([0-9\s]+)+").unwrap();
    let mut numvec: Vec<u64> = Vec::new();
    for cap in nums_re.captures_iter(input) {
        numvec = cap
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
    }
    numvec
}
fn line_has_only_nums(line: &str) -> bool {
    // let no_nums_re = Regex::new(r"^[^\d]+$").unwrap();
    let nums_re = Regex::new(r"([0-9\s]+)+").unwrap();
    nums_re.is_match(line)
}
fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let re = Regex::new(r"seeds:\s([0-9\s]+)+").unwrap();

    let mut seeds: Vec<u64> = Vec::new();

    for cap in re.captures_iter(input) {
        seeds = cap
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
    }
    // println!("{:?}", seeds);

    // Store line numbers of the text markers
    let mut text_marker_indices: Vec<usize> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        // text_marker_indices.push(i);
        match line {
            "seed-to-soil map:"
            | "soil-to-fertilizer map:"
            | "fertilizer-to-water map:"
            | "water-to-light map:"
            | "light-to-temperature map:"
            | "temperature-to-humidity map:"
            | "humidity-to-location map:" => text_marker_indices.push(i),
            _ => continue,
        }
    }
    // println!("{:?}", text_marker_indices);

    let mut all_map_numbers: Vec<Vec<Vec<u64>>> = Vec::new();
    for index in text_marker_indices.iter() {
        let line_iter = input.lines().skip(*index + 1);
        let x2x_lines: Vec<Vec<u64>> = line_iter
            .take_while(|line| line_has_only_nums(line))
            .map(|line| get_numvec_from_string(line))
            .collect();
        all_map_numbers.push(x2x_lines);
    }

    // println!("{:?}", all_map_numbers);
    (seeds, all_map_numbers)
}

fn walk_map(num: u64, map: &Vec<Vec<u64>>) -> u64 {
    for entry in map.iter() {
        let range = entry[1]..(entry[1] + entry[2] + 1);
        if range.contains(&num) {
            return entry[0] + num - entry[1];
        }
    }
    num
}

fn lookup_seed_location(seed: u64, all_map_numbers: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut location: u64 = seed;
    for map in all_map_numbers.iter() {
        location = walk_map(location, map)
    }
    location
}

fn main() {
    // let (seeds, all_test_map_numbers) = parse_input(TEST_INPUT);
    // assert_eq!(walk_map(98, &all_test_map_numbers[0]), 50);
    // assert_eq!(walk_map(53, &all_test_map_numbers[0]), 55);
    // assert_eq!(walk_map(10, &all_test_map_numbers[0]), 10);

    // assert_eq!(lookup_seed_location(79, &all_test_map_numbers), 82);
    // assert_eq!(lookup_seed_location(13, &all_test_map_numbers), 35);

    let (seeds, all_map_numbers) = parse_input(TEST_INPUT);

    let mut all_locations: Vec<u64> = Vec::new();
    for seed in seeds.iter() {
        all_locations.push(lookup_seed_location(*seed, &all_map_numbers));
    }

    // Part 1
    println!(
        "Minimum location amongst all seeds: {}",
        all_locations.iter().min().unwrap()
    );

    // Part 2
    let mut min_location_per_range: Vec<u64> = Vec::new();
    for pair in seeds.chunks(2) {
        println!("{} {}", pair[0], pair[1]);
        let mut locations_per_range: Vec<u64> = Vec::new();
        for seed in pair[0]..(pair[0] + pair[1]) {
            locations_per_range.push(lookup_seed_location(seed, &all_map_numbers));
        }
        let min_location = locations_per_range.iter().min().unwrap();
        min_location_per_range.push(*min_location);
    }

    println!(
        "Minimum location amongst all ranges: {}",
        min_location_per_range.iter().min().unwrap()
    );
}
