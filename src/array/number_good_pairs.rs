// #1512 number of good pairs

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(std::collections::HashMap::new(), |mut map, num| {
                let mut count = map.entry(num).or_insert(0);
                *count += 1;
                map
            })
            .iter()
            .fold(0, |sum, value| sum + value.1 * (value.1 - 1) / 2)
    }
}

#[cfg(test)]
mod test {
    use crate::array::number_good_pairs::Solution;

    #[test]
    pub fn test_num_pairs() {
        let list = vec![1, 2, 3, 1, 1, 3];
        assert_eq!(Solution::num_identical_pairs(list), 4);
    }
}
