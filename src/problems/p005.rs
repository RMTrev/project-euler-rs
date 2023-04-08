use super::RunConfig;
use crate::common::sequence::Prime;

static LIMIT: u64 = 20;

pub fn run(_: RunConfig) {
    let mut sieve = Prime::new();
    let mut result: u64 = 1;

    loop {
        // get next prime
        let p = sieve.next().unwrap();

        // if passed the limit, stop
        if p > LIMIT {
            break;
        }

        // get the highest power for the prime that's still below the limit
        let mut mult = p;
        loop {
            let temp = mult * p;
            if temp > LIMIT {
                break;
            }
            mult = temp;
        }

        // apply multiplier to result
        result *= mult;
    }

    println!("Answer: {}", result);
}
