use super::RunConfig;

static LIMIT: u64 = 1000;

/// Problem 9: Special Pythagorean Triplet
///
/// A Pythagorean triplet is a set of three natural numbers, `a < b < c`, in which
/// ```
/// a^2 + b^2 = c^2
/// ```
///
/// Find a Pythagorean triplet in which `a + b + c = 1000`, and calculate `a * b * c`
pub fn run(_: RunConfig) {
    for a in 1..(LIMIT - 1) {
        for b in 1..(LIMIT - a) {
            let c = LIMIT - (a + b);
            if is_pythagorean_triplet(&a, &b, &c) {
                println!("Answer: {} * {} * {} = {}", a, b, c, a*b*c);
                return;
            }
        }
    }
}

fn is_pythagorean_triplet(a: &u64, b: &u64, c: &u64) -> bool {
    ((a * a) + (b * b)) == (c * c)
}
