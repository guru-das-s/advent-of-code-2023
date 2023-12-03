extern crate regex;

const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test.input");
const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn get_valid_game_number(line: &str) -> (u32, u32) {
    let pattern = regex::Regex::new(r"Game (\d+): (.*)").unwrap();
    let captures = pattern.captures(line).unwrap();

    let game_number = &captures[1];
    let colour_counts = &captures[2];
    println!("{}", line);

    let mut game_valid = true;

    let mut game_red_max: u32 = 0;
    let mut game_green_max: u32 = 0;
    let mut game_blue_max: u32 = 0;

    for group in colour_counts.split(";") {
        for ball_type in group.split(",") {
            let ball_count = ball_type.split_whitespace().nth(0).unwrap().to_string();
            let ball_colour = ball_type.split_whitespace().nth(1).unwrap().to_string();
            if game_valid
                && ball_count_valid(&ball_colour, ball_count.parse::<u32>().unwrap()) == false
            {
                game_valid = false;
            }

            update_max_balls_seen_per_colour(
                &ball_colour,
                ball_count.parse::<u32>().unwrap(),
                &mut game_red_max,
                &mut game_green_max,
                &mut game_blue_max,
            );
        }
    }
    let power = game_red_max * game_green_max * game_blue_max;

    print!("Game {}: ", game_number.to_string().parse::<u32>().unwrap());
    if game_valid == true {
        print!("valid");
        println!(". Power: {}", power);
        return (game_number.to_string().parse::<u32>().unwrap(), power);
    } else {
        print!("invalid");
        println!(". Power: {}", power);
        return (0, power);
    }
}

fn update_max_balls_seen_per_colour(
    colour: &String,
    count: u32,
    red: &mut u32,
    green: &mut u32,
    blue: &mut u32,
) {
    if colour == "red" {
        *red = (*red).max(count);
    } else if colour == "green" {
        *green = (*green).max(count);
    } else if colour == "blue" {
        *blue = (*blue).max(count);
    }
}
fn ball_count_valid(colour: &String, count: u32) -> bool {
    //println!("{} {}", colour, count);
    if colour == "red" {
        count <= MAX_RED
    } else if colour == "green" {
        count <= MAX_GREEN
    } else if colour == "blue" {
        count <= MAX_BLUE
    } else {
        false
    }
}

fn sum_valid_games(input: &str) -> (u32, u32) {
    let all_lines: Vec<_> = input
        .lines()
        .map(|line| get_valid_game_number(line))
        .collect();
    // println!("All lines: {:?}", all_lines);
    let sum_all_valid_games = all_lines.iter().map(|(a, _)| a).sum();
    let sum_all_powers = all_lines.iter().map(|(_, b)| b).sum();

    (sum_all_valid_games, sum_all_powers)
}

fn main() {
    assert_eq!(sum_valid_games(TEST_INPUT), (8, 2286));

    let answers = sum_valid_games(INPUT);

    println!("Part 1 answer: {}, Part 2 answer: {}", answers.0, answers.1);
}
