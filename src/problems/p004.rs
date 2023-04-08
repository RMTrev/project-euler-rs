use super::RunConfig;
use crate::common::num::is_palindromic;

pub fn run(_: RunConfig) {
    let mut largest: u64 = 0;
    for x in 100..1000 {
        for y in x..1000 {
            let n = x * y;
            if is_palindromic(n) && n > largest {
                largest = n;
            }
        }
    }

    println!("Answer: {}", largest);
}
