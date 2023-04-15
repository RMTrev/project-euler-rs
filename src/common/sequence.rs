use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Fibonacci {
    curr: u32,
    next: u32,
    limit: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        if current > self.limit {
            return None;
        }

        Some(current)
    }
}

impl Fibonacci {
    pub fn new(limit: u32) -> Fibonacci {
        Fibonacci {
            curr: 0,
            next: 1,
            limit,
        }
    }
}

/// Iterator implementing an unbounded Sieve of Eratosthenes, adapted from original Java
/// implementation.
///
/// [Source](http://rosettacode.org/wiki/Sieve_of_Eratosthenes#Infinite_iterator_with_a_faster_algorithm_.28sieves_odds-only.29)
pub struct Prime {
    candidate: u64,
    base_primes: Option<Box<RefCell<Prime>>>,
    basep: u64,
    basepsqr: u64,

    /// HashMap to be shared by Prime instances containing all integers marked as non-prime
    nonprimes: Rc<RefCell<HashMap<u64, u64>>>,
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.candidate <= 5 {
            if self.candidate == 2 {
                self.candidate += 1;
                return Some(2);
            }

            self.candidate += 2;
            if self.candidate == 5 {
                return Some(3);
            }

            let baseprimes = RefCell::new(Prime::new_inner(Rc::clone(&self.nonprimes)));
            baseprimes.borrow_mut().next();
            baseprimes.borrow_mut().next();
            self.base_primes = Some(Box::new(baseprimes));
            return Some(5);
        }

        while self.candidate >= self.basepsqr || self.nonprimes.borrow().contains_key(&self.candidate) {
            if self.candidate >= self.basepsqr {
                let adv = self.basep << 1;
                self.nonprimes.borrow_mut().insert(self.basep * self.basep + adv, adv);
                if let Some(bp) = &self.base_primes {
                    if let Some(v) = bp.borrow_mut().next() {
                        self.basep = v;
                    }
                };
                self.basepsqr = self.basep * self.basep;
            } else {
                let adv = self.nonprimes.borrow_mut().remove(&self.candidate).unwrap();
                let mut nxt = self.candidate + adv;
                while self.nonprimes.borrow().contains_key(&nxt) {
                    nxt += adv;
                }

                self.nonprimes.borrow_mut().insert(nxt, adv);
            }

            self.candidate += 2;
        }

        let tmp = self.candidate;
        self.candidate += 2;
        return Some(tmp);
    }
}

impl Prime {
    pub fn new() -> Prime {
        Prime {
            candidate: 2,
            base_primes: None,
            basep: 3,
            basepsqr: 9,
            nonprimes: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn new_inner(nonprimes: Rc<RefCell<HashMap<u64, u64>>>) -> Prime {
        Prime {
            candidate: 2,
            base_primes: None,
            basep: 3,
            basepsqr: 9,
            nonprimes
        }
    }
}

pub struct TriangleNumber {
    curr: u64,
    index: u64,
}

impl Iterator for TriangleNumber {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let returning = self.curr + self.index;
        self.index += 1;
        self.curr = returning;

        Some(returning)
    }
}

impl TriangleNumber {
    pub fn new() -> TriangleNumber {
        TriangleNumber {
            curr: 0,
            index: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_iterator() {
        let first_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
        let mut prime_iter = Prime::new();

        for p in first_primes {
            let test_p = match prime_iter.next() {
                Some(v) => v,
                None => {
                    panic!("This should not happen!");
                }
            };
            assert_eq!(test_p, p);
        }
    }

    #[test]
    fn test_triangle_iterator() {
        let first_entries = vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666];
        let mut t_iter = TriangleNumber::new();

        for t in first_entries {
            let test_tn = match t_iter.next() {
                Some(v) => v,
                None => {
                    panic!("This should not happen!");
                }
            };
            assert_eq!(test_tn, t);
        }
    }
}
