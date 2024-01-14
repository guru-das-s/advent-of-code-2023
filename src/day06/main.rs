const INPUT: &str = include_str!("input");

fn ways_to_win(max_t: i32, max_d: i32) -> i32 {
    // t * (max_t - t) = d
    // -t^2 + max_t * t - d = 0
    // Quadratic equation with a = -1, b = max_t, c = -max_d
    // t = (-b +/- sqrt(b^2 - 4 * a * c)) / (2 * a)

    let discr = max_t.pow(2) - (4 * -1 * -max_d);
    let sqrt_discr = (discr as f64).sqrt();
    let t1: f64 = (-max_t as f64 + sqrt_discr) / (-2 as f64);
    let t2: f64 = (-max_t as f64 - sqrt_discr) / (-2 as f64);
    println!("Roots: {} {}", t1, t2);

    (t2.ceil() - t1.floor() - 1 as f64) as i32
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut times_and_distances: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let mut nums: Vec<i32> = Vec::new();
        nums = line
            .trim()
            .split_whitespace()
            .flat_map(str::parse::<i32>)
            .collect::<Vec<i32>>();
        times_and_distances.push(nums);
    }
    times_and_distances
}

fn main() {
    assert_eq!(ways_to_win(7, 9), 4);
    assert_eq!(ways_to_win(15, 40), 8);
    assert_eq!(ways_to_win(30, 200), 9);

    println!("{:?}", parse_input(INPUT));
}
