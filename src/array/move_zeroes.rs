// # 283 move zeroes

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // let (mut i, mut j) = (0, 0);
        //
        // while i < nums.len() {
        //     while j < nums.len() && nums[j] != 0 {
        //         j += 1;
        //     }
        //
        //     i = j;
        //
        //     while i < nums.len() && nums[i] == 0 {
        //         i += 1;
        //     }
        //
        //     if i < nums.len() {
        //         nums.swap(i, j);
        //     }
        // }

        let (mut i, mut j) = (0, 0);
        while i < nums.len() {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::array::move_zeroes::Solution;

    #[test]
    pub fn test_move_zeros_with_all_zeroes() {
        let mut v = vec![0, 0, 0];
        Solution::move_zeroes(&mut v);
        assert_eq!(&mut v, &mut vec![0, 0, 0]);
    }

    #[test]
    pub fn test_with_none_zero() {
        let mut v = vec![1, 2, 3];
        Solution::move_zeroes(&mut v);
        assert_eq!(&mut v, &mut vec![1, 2, 3]);
    }

    #[test]
    pub fn test_move_zeros() {
        let mut v = vec![1, 2, 0, 0, 0, 3, 0, 0];
        Solution::move_zeroes(&mut v);
        assert_eq!(&mut v, &mut vec![1, 2, 3, 0, 0, 0, 0, 0]);
    }
}
