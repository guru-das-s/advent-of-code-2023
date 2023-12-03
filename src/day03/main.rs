const TEST_INPUT: &str = include_str!("testinput");

fn find_first_ascii_digit_index(data: &[u8]) -> Option<usize> {
    data.iter().position(|&byte| byte >= b'0' && byte <= b'9')
}

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
        let mut start_index = 0;
        while let Some(next_index) = find_first_ascii_digit_index(&row[start_index..]) {
            print!("{} ", start_index + next_index);
            start_index += next_index + 1;
        }
        println!("");
    }
}
