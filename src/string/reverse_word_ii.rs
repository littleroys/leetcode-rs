//#541 reverse string II

struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut v = s.into_bytes();
        let (mut i, n) = (0, v.len());
        let mut reverse = false;
        while i < n {
            let mut start = i;
            while i < n && i < (start + k as usize) {
                i += 1;
            }

            let (mut left, mut right) = (start, i - 1);
            reverse = !reverse;

            while left < right && reverse {
                unsafe {
                    std::ptr::swap(&mut v[left], &mut v[right]);
                }
                left += 1;
                right -= 1;
            }
        }

        String::from_utf8(v).unwrap()
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
