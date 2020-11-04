// # 1480 running sum of list
struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter()
            .map(|num| {
                sum = num + sum;
                sum
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::array::running_sum::Solution;

    #[test]
    pub fn test_running_sum_empty() {
        let num_list = vec![];
        assert_eq!(Solution::running_sum(num_list), vec![])
    }

    #[test]
    pub fn test_running_sum() {
        let num_list = vec![1, 2, 3, 4];
        assert_eq!(Solution::running_sum(num_list), vec![1, 3, 6, 10])
    }
}
