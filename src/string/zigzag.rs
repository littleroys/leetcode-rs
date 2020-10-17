//#6 zigzag

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut result = String::new();
        let delta = num_rows * 2 - 2;
        let limit = s.len() as i32 / delta + 1;

        for r in 0..num_rows {
            for n in 0..limit {
                let idx = r + n * delta;
                let zig_idx = idx + (num_rows - r - 1) * 2;
                if idx < s.len() as i32 {
                    result.push(s.as_bytes()[idx as usize] as char);
                }

                if r!= 0 && r != (num_rows - 1) && zig_idx < s.len() as i32 {
                    result.push(s.as_bytes()[zig_idx as usize] as char);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::string::zigzag::Solution;
    #[test]
    // pub fn test_row_abcde() {
    //     let actual = Solution::convert("ABCDE".to_string(), 4);
    //     assert!(actual.eq("ABCED"));
    // }
    //
    //
    // #[test]
    // pub fn test_row_abcdefg() {
    //     let actual = Solution::convert("ABCDEFG".to_string(), 4);
    //     assert!(actual.eq("AGBFCED"));
    // }
    //
    // #[test]
    // pub fn test_row_3() {
    //     let actual = Solution::convert("PAYPALISHIRING".to_string(), 3);
    //     assert!(actual.eq("PAHNAPLSIIGYIR"));
    // }

    #[test]
    pub fn test_row_4() {
        let actual = Solution::convert("PAYPALISHIRING".to_string(), 4);
        assert!(actual.eq("PINALSIGYAHRPI"));
    }
}
