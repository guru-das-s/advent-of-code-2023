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
    let mut s2s = 0;
    let mut s2f = 0;
    let mut f2w = 0;
    let mut w2l = 0;
    let mut l2t = 0;
    let mut t2h = 0;
    let mut h2L = 0;
    for (i, line) in input.lines().enumerate() {
        if line == "seed-to-soil map:" {
            s2s = i;
            continue;
        }
        if line == "soil-to-fertilizer map:" {
            s2f = i;
            continue;
        }
        if line == "fertilizer-to-water map:" {
            f2w = i;
            continue;
        }
        if line == "water-to-light map:" {
            w2l = i;
            continue;
        }
        if line == "light-to-temperature map:" {
            l2t = i;
            continue;
        }
        if line == "temperature-to-humidity map:" {
            t2h = i;
            continue;
        }
        if line == "humidity-to-location map:" {
            h2L = i;
        }
    }
    println!("{} {} {} {} {} {} {}", s2s, s2f, f2w, w2l, l2t, t2h, h2L);

    let line_iter = input.lines().skip(s2s + 1);
    let s2s_lines: Vec<Vec<u32>> = line_iter
        .take_while(|line| line_has_only_nums(line))
        .map(|line| get_numvec_from_string(line))
        .collect();

    println!("{:?}", s2s_lines);
}

fn main() {
    parse_input(TEST_INPUT);
}
