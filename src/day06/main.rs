fn ways_to_win(max_t: u32, max_d: u32) -> usize {
    let mut ways: Vec<u32> = Vec::new();

    ways = (1..max_t)
        .step_by(1)
        .filter_map(|t| {
            if (max_t - t) > (max_d / t) {
                Some(t)
            } else {
                None
            }
        })
        .collect();

    println!("{:?}, len {}", ways, ways.len());

    ways.len()
}

fn main() {
    assert_eq!(ways_to_win(7, 9), 4);
    assert_eq!(ways_to_win(15, 40), 8);
    assert_eq!(ways_to_win(30, 200), 9);
}
