// #1470

struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(2 * n as usize);
        (0..n).for_each(|idx| {
            result.push(nums[idx as usize]);
            result.push(nums[(n + idx) as usize]);
        });

        result
    }
}

#[cfg(test)]
mod test {
    use crate::array::shuffle_arrays::Solution;

    #[test]
    pub fn test_shuffle() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(Solution::shuffle(nums, 4), vec![1, 5, 2, 6, 3, 7, 4, 8]);
    }
}
