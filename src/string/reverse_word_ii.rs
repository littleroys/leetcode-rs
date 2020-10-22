//#541 reverse string II

struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::string::reverse_word_ii::Solution;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::reverse_str("abcdefg".to_string(), 2),
            "bacdfeg".to_string()
        );
    }
}
