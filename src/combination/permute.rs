/// Heap's Algorithm
///
/// #46 Permutations
///
struct Solution;

impl Solution {
    pub fn permute(nums: &mut Vec<i32>, n: i32) {
        let result: Vec<Vec<i32>> = Vec::from(Vec::from());
        if n == 1 {
            println!("{:?}", nums);
            return;
        }

        for i in 0..nums.len() {
            Solution::permute(nums, n - 1);
            if i % 2 != 0 {
                // odd swap 0 and last
                nums.swap(0, (n - 1) as usize)
            } else {
                // even swap current and last
                nums.swap(i, (n - 1) as usize)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::combination::permute::Solution;

    #[test]
    pub fn test_permute() {
        let mut nums = vec![1, 2, 3];
        Solution::permute(&mut nums, 3)
    }
}
