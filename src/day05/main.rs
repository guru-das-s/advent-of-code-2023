use regex::Regex;

const TEST_INPUT: &str = include_str!("testinput");

fn get_numvec_from_string(input: &str) -> Vec<u32> {
    let nums_re = Regex::new(r"([0-9\s]+)+").unwrap();
    let mut numvec: Vec<u32> = Vec::new();
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
    println!("{:?}", text_marker_indices);

    let line_iter = input.lines().skip(text_marker_indices[0] + 1);
    let s2s_lines: Vec<Vec<u32>> = line_iter
        .take_while(|line| line_has_only_nums(line))
        .map(|line| get_numvec_from_string(line))
        .collect();

    println!("{:?}", s2s_lines);
}

fn main() {
    parse_input(TEST_INPUT);
}
