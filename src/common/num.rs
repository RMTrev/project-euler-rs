pub fn is_palindromic(n: u64) -> bool {
    let n = n.to_string();

    let len: usize = n.len();
    let mid: usize = n.len() / 2;
    let n = n.as_bytes();

    for i in 0..mid {
        let ch1: u8 = n[i];
        let ch2: u8 = n[len - (i + 1)];
        if ch1 != ch2 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(is_palindromic(1001), true);
        assert_eq!(is_palindromic(10101), true);
        assert_eq!(is_palindromic(12321), true);
        assert_eq!(is_palindromic(12331), false);
        assert_eq!(is_palindromic(1), true);
        assert_eq!(is_palindromic(12), false);
        assert_eq!(is_palindromic(11), true);
    }
}
