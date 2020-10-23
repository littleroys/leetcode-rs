// #88 merge sorted array
struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut p1, mut p2) = (m - 1, n - 1);
        let mut p = m + n - 1;
        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] < nums2[p2 as usize] {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1
            } else {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1
            }
            p -= 1
        }

        // copy rest p2 vec
        for i in 0..p2 + 1 {
            nums1[i as usize] = nums2[i as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::array::merge_sort_array::Solution;

    #[test]
    pub fn test_merge_sorted_str() {
        let mut v1 = vec![1, 2, 3, 0, 0, 0];
        Solution::merge(&mut v1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(v1, vec![1, 2, 2, 3, 5, 6])
    }

    #[test]
    pub fn test_merge_sorted_str1() {
        let mut v1 = vec![7, 0, 0, 0];
        Solution::merge(&mut v1, 1, &mut vec![2, 5, 6], 3);

        assert_eq!(v1, vec![2, 5, 6, 7])
    }
}
