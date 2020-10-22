// #231 Power of two

struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0)
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::power_of_two::Solution;

    #[test]
    pub fn test() {
        assert_eq!(Solution::is_power_of_two(4), true);
        assert_eq!(Solution::is_power_of_two(5), false);
        assert_eq!(Solution::is_power_of_two(16), true);
    }
}
