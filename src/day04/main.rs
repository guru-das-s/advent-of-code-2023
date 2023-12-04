use regex::Regex;

extern crate regex;

const TEST_INPUT: &str = include_str!("testinput");
const INPUT: &str = include_str!("input");

fn calculate_total_points(all_cards: &str) -> u32 {
    let mut total_points = 0;
    let re = Regex::new(r": ([0-9\s]+)+\|([0-9\s]+)+").unwrap();
    for card in all_cards.lines() {
        for cap in re.captures_iter(card) {
            let left_numbers: Vec<u32> = cap
                .get(1)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let right_numbers: Vec<u32> = cap
                .get(2)
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let common: Vec<u32> = left_numbers
                .iter()
                .filter(|&num| right_numbers.contains(num))
                .cloned()
                .collect();
            // println!("Matches: {:?}", common);
            let base: u32 = 2;
            let len = common.len() as u32;
            if len > 0 {
                total_points += base.pow(len - 1);
            }
        }
    }
    total_points
}

fn main() {
    assert_eq!(calculate_total_points(TEST_INPUT), 13);

    println!("Part 1: Total points: {}", calculate_total_points(INPUT));
}
