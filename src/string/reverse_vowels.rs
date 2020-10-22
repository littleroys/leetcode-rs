// #345 reverse vowels of string
struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::string::reverse_vowels::Solution;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::reverse_vowels("hello".to_string()),
            "holle".to_string()
        );
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }
}
