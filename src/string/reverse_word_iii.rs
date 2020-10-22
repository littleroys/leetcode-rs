// #557 reverse words in a string III
use std::mem::swap;

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s

        // s.split_whitespace()
        //     .map(|word| word.chars().rev().collect::<String>() + " ")
        //     .collect::<String>()
        //     .trim()
        //     .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::string::reverse_word_iii::Solution;

    #[test]
    pub fn test() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL   ekat edoCteeL tsetnoc".to_string()
        );
    }
}
