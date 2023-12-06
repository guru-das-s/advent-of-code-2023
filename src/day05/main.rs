use regex::Regex;

const TEST_INPUT: &str = include_str!("testinput");

fn parse_input(input: &str) {
    let re = Regex::new(r"seeds:\s([0-9\s]+)+").unwrap();
    let mut seeds: Vec<u32> = Vec::new();

    for cap in re.captures_iter(input) {
        seeds = cap
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
    }
    println!("{:?}", seeds);
}

fn main() {
    parse_input(TEST_INPUT);
}
