// #151 reverse words
struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod test {
    use crate::string::reverse_word::Solution;

    #[test]
    pub fn test_empty_str() {
        assert_eq!(Solution::reverse_words("".to_string()), "".to_string());
    }

    #[test]
    pub fn test_reverse_word() {
        assert_eq!(
            Solution::reverse_words("   the sky  is blue ".to_string()),
            "blue is sky the".to_string()
        );

        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello".to_string()
        );
    }
}
