const TEST_INPUT: &str = include_str!("testinput");

fn check_if_part_num(
    schematic: &Vec<Vec<char>>,
    row: usize,
    col_start: usize,
    col_end: usize,
) -> bool {
    let height = schematic.len();
    let width = schematic[0].len();

    let row_up = if row > 0 { row - 1 } else { 0 };
    let row_down = (row + 1).min(height);
    let col_left = if col_start > 0 { col_start - 1 } else { 0 };
    let col_right = (col_end + 1).min(width);

    print!("({} {})({} {}): ", row_up, col_left, row_down, col_right);

    for i in row_up..row_down {
        for j in col_left..col_right {
            let c = schematic[i][j];
            if !c.is_ascii_digit() && c != '.' {
                // Yay! It's a part number.
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in TEST_INPUT.lines() {
        schematic.push(line.chars().collect::<Vec<_>>());
    }

    let height = schematic.len();
    let width = schematic[0].len();

    let mut row: usize = 0;
    let mut col: usize = 0;

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
            while schematic[row][col].is_ascii_digit() {
                col = col + 1;
            }
            num_end = col;
            let num: u32 = schematic[row][num_start..num_end]
                .iter()
                .copied()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            print!("{}: ", num);
            let is_part_num = check_if_part_num(&schematic, row, num_start, num_end);
            print!("{}, ", is_part_num);
            col = col + 1;
        }
        row = row + 1;
        col = 0;
        println!("");
    }
}
