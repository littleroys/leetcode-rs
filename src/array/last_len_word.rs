// #58
struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let word_list: Vec<&str> = s.trim().rsplit(' ').collect();
        if word_list.is_empty() {
            0
        } else {
            word_list[0].len() as i32
        }
    }
}

#[cfg(test)]
mod test {
    use crate::array::last_len_word::Solution;

    #[test]
    pub fn test_last_word() {
        assert_eq!(Solution::length_of_last_word("hello world".to_string()), 5)
    }

    #[test]
    pub fn test_one_word() {
        assert_eq!(Solution::length_of_last_word("a".to_string()), 1)
    }

    #[test]
    pub fn test_empty() {
        assert_eq!(Solution::length_of_last_word("".to_string()), 0)
    }

}