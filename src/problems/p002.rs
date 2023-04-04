use super::RunConfig;
use crate::common::sequence::Fibonacci;

pub fn run(_: RunConfig) {
    let mut fibo = Fibonacci::new(4000000);
    let mut result: u32 = 0;

    for n in fibo {
        if n % 2 == 0 {
            result += n;
        }
    }

    println!("Answer: {}", result);
}
