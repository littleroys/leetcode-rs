// #79 word search
struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let idx = 0;
        for row in board {
            for ch in row {}
        }

        true
    }
}

#[cfg(test)]
mod test {
    use crate::array::word_search::Solution;

    #[test]
    pub fn test_exist() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true
        )
    }
}
