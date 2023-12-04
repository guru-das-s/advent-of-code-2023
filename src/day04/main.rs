use regex::Regex;

extern crate regex;

const TEST_INPUT: &str = include_str!("testinput");
const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Card {
    count: u32,
    matches: u32,
}

fn calculate_total_points(all_cards: &str, card_db: &mut Vec<Card>) -> u32 {
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
            card_db.push(Card {
                count: 1,
                matches: len,
            });
            if len > 0 {
                total_points += base.pow(len - 1);
            }
        }
    }
    total_points
}

fn win_more_cards(card_db: &mut Vec<Card>) {
    let mut i: usize = 0;

    while i < card_db.len() {
        let current_card_matches = card_db[i].matches;
        let current_card_count = card_db[i].count;
        for j in (i + 1)..(i + current_card_matches as usize + 1) {
            card_db[j].count += current_card_count;
        }
        i += 1;
    }
}

fn get_total_card_count(card_db: &Vec<Card>) -> u32 {
    let mut new_card_total = 0;

    for card in card_db.iter() {
        new_card_total += card.count;
    }

    new_card_total
}

fn main() {
    let mut test_card_db: Vec<Card> = Vec::new();
    assert_eq!(calculate_total_points(TEST_INPUT, &mut test_card_db), 13);
    println!("{:?}", test_card_db);

    win_more_cards(&mut test_card_db);
    println!("{:?}", test_card_db);

    assert_eq!(get_total_card_count(&test_card_db), 30);

    let mut card_db: Vec<Card> = Vec::new();
    println!(
        "Part 1: Total points: {}",
        calculate_total_points(INPUT, &mut card_db)
    );
    win_more_cards(&mut card_db);

    println!(
        "Part 2: Total new number of cards: {}",
        get_total_card_count(&card_db)
    );
}
