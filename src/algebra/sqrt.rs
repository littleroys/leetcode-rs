// #69 sqrt(x)
use std::num;

struct Solution;

impl Solution {
    // Newton's method
    pub fn my_sqrt(x: i32) -> i32 {
        let mut k: f64 = 1.0;
        let epsilon: f64 = 0.1;
        while (k * k - x as f64).abs() > epsilon {
            k = 0.5 * (k + x as f64 / k);
        }

        k as i32
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::sqrt::Solution;

    #[test]
    pub fn test_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(2147395599), 46339)
    }
}
