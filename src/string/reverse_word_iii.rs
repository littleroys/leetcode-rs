// #557 reverse words in a string III
struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut v = s.into_bytes();
        let (mut right, mut left) = (0, 0);

        while right < v.len() {
            while right < v.len() && v[right] != ' ' as u8 {
                right += 1;
            }

            let next = right + 1;

            while left < right {
                unsafe {
                    std::ptr::swap(&mut v[right - 1], &mut v[left]);
                }
                left += 1;
                right -= 1;
            }
            left = next;
            right = next;
        }

        String::from_utf8(v).unwrap()

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
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
    }
}
