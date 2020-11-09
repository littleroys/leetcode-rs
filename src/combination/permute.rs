struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::combination::permute::Solution;

    #[test]
    pub fn test_permute() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::permute(nums), vec![vec![1, 2, 3], vec![3, 2, 1],]);
    }
}
