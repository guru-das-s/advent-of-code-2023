use std::collections::HashMap;

const TEST_INPUT: &str = include_str!("testinput");
const INPUT: &str = include_str!("input");

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

fn check_if_part_num(
    schematic: &Vec<Vec<char>>,
    row: usize,
    col_start: usize,
    col_end: usize,
) -> (bool, bool, Option<Coord>) {
    let height = schematic.len() - 1;
    let width = schematic[0].len() - 1;

    let row_up = if row > 0 { row - 1 } else { 0 };
    let row_down = (row + 1).min(height);
    let col_left = if col_start > 0 { col_start - 1 } else { 0 };
    let col_right = (col_end + 1).min(width);

    print!("({} {})({} {}): ", row_up, col_left, row_down, col_right);

    for i in row_up..(row_down + 1) {
        for j in col_left..(col_right + 1) {
            let c = schematic[i][j];
            if !c.is_ascii_digit() && c != '.' {
                // Yay! It's a part number.
                if c == '*' {
                    return (true, true, Some(Coord { row: i, col: j }));
                } else {
                    return (true, false, None);
                }
            }
        }
    }

    (false, false, None)
}

fn main() {
    let mut schematic: Vec<Vec<char>> = Vec::new();
    let mut nums_around_star: HashMap<Coord, Vec<u32>> = HashMap::new();

    for line in INPUT.lines() {
        schematic.push(line.chars().collect::<Vec<_>>());
    }

    let height = schematic.len();
    let width = schematic[0].len();

    let mut row: usize = 0;
    let mut col: usize = 0;

    let mut sum_of_parts = 0;

    while row < height {
        while col < width {
            let c = schematic[row][col];
            if !c.is_ascii_digit() {
                col = col + 1;
                continue;
            }

            // Number begins
            let num_start = col;

            // Find its end
            let mut num_end = 0;
            while col < width && schematic[row][col].is_ascii_digit() {
                col = col + 1;
            }

            // // Guard against overflow - cap at max width index
            // col = col.min(width - 1);

            num_end = col;
            let num: u32 = schematic[row][num_start..num_end]
                .iter()
                .copied()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            print!("{}: ", num);
            num_end -= 1;
            let (is_part_num, is_star, star_coord_option) =
                check_if_part_num(&schematic, row, num_start, num_end);
            let print_star = if is_star { "[*] " } else { " " };
            print!("{}{}", is_part_num, print_star);
            if is_part_num == true {
                sum_of_parts += num;
                if is_star == true {
                    let star_coord = star_coord_option.unwrap();
                    let num_vec = nums_around_star
                        .entry(star_coord)
                        .or_insert_with(|| Vec::new());
                    num_vec.push(num);
                }
            }
            col = col + 1;
        }
        row = row + 1;
        col = 0;
        println!("");
    }

    println!("Sum of all valid part numbers: {}", sum_of_parts);
    println!("{:?}", nums_around_star);
    let mut gear_ratio_sum = 0;
    for (_, numvec) in nums_around_star.iter() {
        if numvec.len() == 2 {
            gear_ratio_sum += numvec[0] * numvec[1];
        }
    }
    println!("Sum of all gear ratios: {}", gear_ratio_sum);
}
