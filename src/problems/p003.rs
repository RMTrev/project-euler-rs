use super::RunConfig;
use crate::common::sequence::Prime;

static BIG_NUMBER: u64 = 600851475143;

pub fn run(_: RunConfig) {
    let p = Prime::new();
    let mut limit: u64 = BIG_NUMBER;
    let mut result: u64 = 0;

    for n in p {
        if limit % n == 0 {
            println!("{} is a prime factor of {}", n, BIG_NUMBER);
            result = n;
            limit /= n;
            println!("New limit: {}", limit);
        }

        if n >= limit {
            break;
        }
    }

    println!("Answer: {}", result);
}
