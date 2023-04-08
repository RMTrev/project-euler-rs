use super::RunConfig;
use crate::common::sequence::Prime;

static LIMIT: u64 = 10001;

/// Problem 7: Get the 10,001st prime
pub fn run(_: RunConfig) {
    let mut sieve = Prime::new();
    let mut p: u64 = 0;

    for _ in 0..LIMIT {
        p = sieve.next().unwrap();
    }

    println!("Answer: {}", p);
}
