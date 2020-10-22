struct Solution;

impl Solution {
    pub fn power_of_four(num: i32) -> bool {
        num > 0 && (num & (num - 1) == 0) && (num as u32 & 0xaaaaaaaau32 == 0)
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::power_of_four::Solution;

    #[test]
    pub fn test() {
        assert_eq!(Solution::power_of_four(4), true);
        assert_eq!(Solution::power_of_four(8), false);
        assert_eq!(Solution::power_of_four(16), true);
    }
}
