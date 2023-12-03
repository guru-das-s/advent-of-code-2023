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
            print!("{}", schematic[row][col]);
            col = col + 1;
        }
        row = row + 1;
        col = 0;
        println!("");
    }
}
