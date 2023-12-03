const TEST_INPUT: &str = include_str!("testinput");

fn main() {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in TEST_INPUT.lines() {
        schematic.push(line.chars().collect::<Vec<_>>());
    }

    let height = schematic.len();
    let width = schematic[0].len();

    println!("Height: {}, width: {}", height, width);

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
            print!("{} ", num);
            col = col + 1;
        }
        row = row + 1;
        col = 0;
        println!("");
    }
}
