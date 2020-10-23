// #832 Flipping an Image;

struct Solution;

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut a = a;
        let n = a[0].len();
        for mut v in a.iter_mut() {
            for i in 0..(n + 1) / 2 {
                let tmp = v[i] ^ 1;
                v[i] = v[n - 1 - i] ^ 1;
                v[n - 1 - i] = tmp;
            }
        }
        a

        // a.into_iter()
        //     .map(|v| v.into_iter().rev().map(|i| i ^ 1).collect())
        //     .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::array::flip_and_invert_image::Solution;

    #[test]
    pub fn test_flipping_1x1() {
        let matrix = vec![vec![1]];
        let result = Solution::flip_and_invert_image(matrix);
    }

    #[test]
    pub fn test_flipping() {
        let matrix = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let result = Solution::flip_and_invert_image(matrix);
    }

}
