// #59 spiral matrix

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let result = Vec::new();
        result
    }
}

#[cfg(test)]
mod test {
    use crate::array::spiral_matrix::Solution;

    #[test]
    pub fn test_spiral_empty() {
        let matrix = vec![];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result.is_empty(), true)
    }

    #[test]
    pub fn test_spiral_1x1() {
        let matrix = vec![vec![1]];
        let result = Solution::spiral_order(matrix);

        assert_eq!(result[0], 1)
    }

    #[test]
    pub fn test_spiral_3x1() {
        let matrix = vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]];
        let result = Solution::spiral_order(matrix);

        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 3);
        assert_eq!(result[3], 4);
    }

    #[test]
    pub fn test_spiral_3x3() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::spiral_order(matrix);

        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 3);
        assert_eq!(result[3], 6);
        assert_eq!(result[4], 9);
        assert_eq!(result[5], 8);
        assert_eq!(result[6], 7);
        assert_eq!(result[7], 4);
        assert_eq!(result[8], 5);
    }

    #[test]
    pub fn test_spiral_4x5() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
        ];
        let result = Solution::spiral_order(matrix);

        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 3);
        assert_eq!(result[3], 4);
    }
}
