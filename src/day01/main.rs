use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut d1 = 0;
    let mut d2 = 0;
    let mut total = 0;
    let word_to_digit = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
        ("ten", "t10n"),
    ]);

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("src/day01/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip2) = line {
                let mut ip = ip2;
                // First process the string
                for (word, digit) in word_to_digit.iter() {
                    ip = ip.replace(word, digit);
                }

                for c in ip.chars() {
                    if c.is_digit(10) {
                        d1 = c.to_digit(10).unwrap();
                        break;
                    }
                }

                for c in ip.chars().rev() {
                    if c.is_digit(10) {
                        d2 = c.to_digit(10).unwrap();
                        break;
                    }
                }

                //println!("{}", d1 * 10 + d2);
                total += d1 * 10 + d2;
            }
        }
    }
    println!("{}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
