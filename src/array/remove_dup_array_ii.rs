// #80 Remove Duplicates from Sorted Array II

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut i, mut count) = (0 as i32, nums.len() as i32);
        while i < nums.len() as i32 {
            let mut start = i;

            while i < nums.len() as i32 && nums[i as usize] == nums[start as usize] {
                i += 1;
            }

            let delta: i32 = (i - start - 2) as i32;

            if delta > 0 {
                count = count - delta;
                for idx in 0..delta {
                    nums.remove((start + idx) as usize);
                }
                i = i - delta;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use crate::array::remove_dup_array_ii::Solution;

    #[test]
    pub fn test_remove() {
        // let mut v1 = vec![1, 1, 1, 2, 2, 3];
        // assert_eq!(Solution::remove_duplicates(&mut v1), 5);
        // println!("{:?}", v1);

        // let mut v2 = vec![0, 0, 1, 1, 1, 1, 1, 2, 2, 3, 3];
        // assert_eq!(Solution::remove_duplicates(&mut v2), 8);

        let mut v3 = vec![
            -41, -40, -40, -40, -40, -40, -40, -39, -38, -38, -38, -38, -37,
        ];

        Solution::remove_duplicates(&mut v3);

        println!("{:?}", v3)
    }

    #[test]
    pub fn test() {
        let mut foo = vec![1, 2, 3, 4, 5];
        let mut i = 0;
        while i < foo.len() {
            foo.swap_remove(0);
            foo.swap_remove(0);
            i += 1;
        }
    }
}

// [-50,-50,-49,-48,-47,-47,-47,-46,-45,-43,-42,-41,-40,-40,-40,-40,-40,-40,-39,-38,-38,-38,-38,-37,-36,-35,-34,-34,-34,-33,-32,-31,-30,-28,-27,-26,-26,-26,-25,-25,-24,-24,-24,-22,-22,-21,-21,-21,-21,-21,-20,-19,-18,-18,-18,-17,-17,-17,-17,-17,-16,-16,-15,-14,-14,-14,-13,-13,-12,-12,-10,-10,-9,-8,-8,-7,-7,-6,-5,-4,-4,-4,-3,-1,1,2,2,3,4,5,6,6,7,8,8,9,9,10,10,10,11,11,12,12,13,13,13,14,14,14,15,16,17,17,18,20,21,22,22,22,23,23,25,26,28,29,29,29,30,31,31,32,33,34,34,34,36,36,37,37,38,38,38,39,40,40,40,41,42,42,43,43,44,44,45,45,45,46,47,47,47,47,48,49,49,49,50]
// [-50,-50,-49,-48,-47,-47,-46,-45,-43,-42,-41,-40,-40,-40,-38,-38,-37,-36,-35,-34,-34,-33,-32,-31,-30,-28,-27,-26,-26,-25,-25,-24,-24,-22,-22,-21,-21,-20,-19,-18,-18,-17,-17,-16,-16,-15,-14,-14,-13,-13,-12,-12,-10,-10,-9,-8,-8,-7,-7,-6,-5,-4,-4,-3,-1,1,2,2,3,4,5,6,6,7,8,8,9,9,10,10,11,11,12,12,13,13,14,14,15,16,17,17,18,20,21,22,22,23,23,25,26,28,29,29,30,31,31,32,33,34,34,36,36,37,37,38,38,39,40,40,41,42,42,43,43,44,44,45,45,46,47,47,48,49,49,50]
// [-50,-50,-49,-48,-47,-47,-46,-45,-43,-42,-41,-40,-40,-39,-38,-38,-37,-36,-35,-34,-34,-33,-32,-31,-30,-28,-27,-26,-26,-25,-25,-24,-24,-22,-22,-21,-21,-20,-19,-18,-18,-17,-17,-16,-16,-15,-14,-14,-13,-13,-12,-12,-10,-10,-9,-8,-8,-7,-7,-6,-5,-4,-4,-3,-1,1,2,2,3,4,5,6,6,7,8,8,9,9,10,10,11,11,12,12,13,13,14,14,15,16,17,17,18,20,21,22,22,23,23,25,26,28,29,29,30,31,31,32,33,34,34,36,36,37,37,38,38,39,40,40,41,42,42,43,43,44,44,45,45,46,47,47,48,49,49,50]

// [-50, -50, -49, -48, -47, -47, -46, -45, -43, -42, -41, -40, -40, -40, -38, -38, -37, -36, -35, -34, -34, -33, -32, -31, -30, -28, -27, -26, -26, -25, -25, -24, -24, -22, -22, -21, -21, -20, -19, -18, -18, -17, -17, -16, -16, -15, -14, -14, -13, -13, -12, -12, -10, -10, -9, -8, -8, -7, -7, -6, -5, -4, -4, -3, -1, 1, 2, 2, 3, 4, 5, 6, 6, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 16, 17, 17, 18, 20, 21, 22, 22, 23, 23, 25, 26, 28, 29, 29, 30, 31, 31, 32, 33, 34, 34, 36, 36, 37, 37, 38, 38, 39, 40, 40, 41, 42, 42, 43, 43, 44, 44, 45, 45, 46, 47, 47, 48, 49, 49, 50]
