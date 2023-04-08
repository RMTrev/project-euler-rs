use super::RunConfig;
use crate::common::sequence::Prime;

static LIMIT: u64 = 20;

pub fn run(_: RunConfig) {
    let mut sieve = Prime::new();
    let mut result: u64 = 1;

    loop {
        let p = sieve.next().unwrap();

        if p > LIMIT {
            break;
        }

        let mut mult = p;
        loop {
            let temp = mult * p;
            if temp > LIMIT {
                break;
            }
            mult = temp;
        }

        result *= mult;
    }

    println!("Answer: {}", result);
}
