use super::RunConfig;
use crate::common::sequence::Prime;

static LIMIT: u64 = 2000000;

/// Problem 10: Summation of primes
///
/// Find the sum of all primes below two million.
pub fn run(_: RunConfig) {
    let mut sieve = Prime::new();
    let mut sum: u64 = 0;

    loop {
        let p = sieve.next().unwrap();
        if p > LIMIT {
            break;
        }

        sum += p;
    }

    println!("Answer: {}", sum);
}
