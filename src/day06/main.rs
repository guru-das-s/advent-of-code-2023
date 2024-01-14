const INPUT: &str = include_str!("input");

fn smoosh_num_vec(nums: &Vec<i64>) -> i64 {
    let mut smoosh = String::new();

    for num in nums {
        smoosh += &num.to_string();
    }

    smoosh.parse().unwrap()
}

fn ways_to_win(max_t: i64, max_d: i64) -> i64 {
    // t * (max_t - t) = d
    // -t^2 + max_t * t - d = 0
    // Quadratic equation with a = -1, b = max_t, c = -max_d
    // t = (-b +/- sqrt(b^2 - 4 * a * c)) / (2 * a)

    let discr = max_t.pow(2) - (4 * -1 * -max_d);
    let sqrt_discr = (discr as f64).sqrt();
    let t1: f64 = (-max_t as f64 + sqrt_discr) / (-2 as f64);
    let t2: f64 = (-max_t as f64 - sqrt_discr) / (-2 as f64);
    println!("Roots: {} {}", t1, t2);

    (t2.ceil() - t1.floor() - 1 as f64) as i64
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut times_and_distances: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let mut nums: Vec<i64> = Vec::new();
        nums = line
            .trim()
            .split_whitespace()
            .flat_map(str::parse::<i64>)
            .collect::<Vec<i64>>();
        times_and_distances.push(nums);
    }
    times_and_distances
}

fn main() {
    assert_eq!(ways_to_win(7, 9), 4);
    assert_eq!(ways_to_win(15, 40), 8);
    assert_eq!(ways_to_win(30, 200), 9);

    let times_and_distances = parse_input(INPUT);
    let num_pairs = times_and_distances[0].len();
    let mut all_ways_product = 1;

    for i in 0..num_pairs {
        all_ways_product *= ways_to_win(times_and_distances[0][i], times_and_distances[1][i]);
    }

    println!("Part 1: {}", all_ways_product);

    assert_eq!(ways_to_win(71530, 940200), 71503);

    let dekerned_time = smoosh_num_vec(&times_and_distances[0]);
    let dekerned_distance = smoosh_num_vec(&times_and_distances[1]);

    println!("Part 2: {}", ways_to_win(dekerned_time, dekerned_distance));
}
