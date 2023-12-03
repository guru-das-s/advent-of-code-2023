const TEST_INPUT: &str = include_str!("testinput");

fn main() {
    let mut schematic: Vec<Vec<u8>> = Vec::new();

    for line in TEST_INPUT.lines() {
        schematic.push(line.as_bytes().to_vec());
    }

    let height = schematic.len();
    let width = schematic[0].len();

    println!("Height: {}, width: {}", height, width);

    for (i, row) in schematic.iter().enumerate() {
        print!("Row {}: ", i);
        if let Some(num) = row.iter().position(|&x| x >= b'0' && x <= b'9') {
            println!("{}", num);
        } else {
            println!("X");
        }
    }
}
