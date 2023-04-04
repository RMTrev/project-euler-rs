use super::RunConfig;

pub fn run(_: RunConfig) {
    let mut result: u32 = 0;

    for n in 0..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            result += n;
        }
    }

    println!("Answer: {}", result);
}
