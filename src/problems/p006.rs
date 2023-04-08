use super::RunConfig;

static LIMIT: u64 = 100;

pub fn run(_: RunConfig) {
    let mut sum: u64 = 0;
    let mut sum_sq: u64 = 0;

    for n in 1..(LIMIT + 1) {
        sum += n;
        sum_sq += n * n;
    }

    let answer = (sum * sum) - sum_sq;
    println!("Answer: {}", answer);
}
