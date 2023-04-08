use super::RunConfig;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use regex::Regex;

static DIGITS: usize = 13;

/// Problem 8: Largest product in a series
/// Find the thirteen adjacent digits in the 1000 digit number that have the greatest product.
///
/// NOTE: the usage for this problem is:
/// ```
/// cargo run 8 filename
/// ```
/// where `filename` is the path to a text file containing the 1000 digit number. The data
/// provided by Project Euler is located in `data/p008-big-number.txt`.
pub fn run(config: RunConfig) {
    if config.params.len() < 1 {
        eprintln!("Problem 8 requires a filename");
    }

    let filename = &config.params[0];
    let file = File::open(filename).unwrap();
    let mut num = String::new();

    // regex to validate our input
    let re = Regex::new(r"^\d+$").unwrap();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        assert!(re.is_match(&line));

        num.push_str(line.trim());
    }

    let num = num.as_bytes();
    let len = num.len();

    let mut largest: u64 = 0;

    for idx in 0..(len - DIGITS) {
        let mut temp: u64 = 1;
        for n in 0..DIGITS {
            let digit = (num[idx + n] as u64) - 48;
            temp *= digit;
        }

        if temp > largest {
            largest = temp;
        }
    }

    println!("Answer: {}", largest);
}
