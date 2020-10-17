// #1 two sum

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut target_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (idx, ele) in nums.iter().enumerate() {
            if target_map.contains_key(ele) {
                return vec![*target_map.get(ele).unwrap(), idx as i32];
            }
            target_map.entry(target - ele).or_insert(idx as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::array::two_sum::Solution;

    #[test]
    pub fn test() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        println!("{:?}", result);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2)
    }
}
