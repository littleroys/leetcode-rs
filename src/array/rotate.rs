// #48 rotate image

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in i..(n - i - 1) {
                let mut new_i_j = (i, j);
                let mut old = matrix[i][j];
                for _ in 0..4 {
                    new_i_j = (new_i_j.1, n - 1 - new_i_j.0);
                    let (new_i, new_j) = new_i_j;
                    std::mem::swap(&mut matrix[new_i][new_j], &mut old);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::array::rotate::Solution;

    #[test]
    pub fn test_rotate_1x1() {
        let mut matrix = vec![vec![1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix[0][0], 1);
    }

    #[test]
    pub fn test_rotate_3x3() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);

        assert_eq!(matrix[0][0], 7);
        assert_eq!(matrix[0][1], 4);
        assert_eq!(matrix[0][2], 1);
        assert_eq!(matrix[1][0], 8);
        assert_eq!(matrix[1][1], 5);
        assert_eq!(matrix[1][2], 2);
        assert_eq!(matrix[2][0], 9);
        assert_eq!(matrix[2][1], 6);
        assert_eq!(matrix[2][2], 3);
    }
}
