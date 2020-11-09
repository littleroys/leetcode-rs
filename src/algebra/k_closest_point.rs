// 973 find k-th close points
struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points: Vec<Vec<i32>> = points;
        points.sort_by(|a, b| (a[0].pow(2) + a[1].pow(2))
            .cmp(&(b[0].pow(2) + b[1].pow(2))));

        points.drain(..k as usize).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::algebra::k_closest_point::Solution;

    #[test]
    pub fn test_k_closest() {
        assert_eq!(Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
                   vec![vec![3, 3], vec![-2, 4]])
    }
}
