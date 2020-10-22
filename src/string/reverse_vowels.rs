// #345 reverse vowels of string
struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let vowels = "aeiouAEIOU";
        let mut v = s.into_bytes();
        let (mut i, mut j) = (1, v.len() as i32);
        while i < j {
            while i < j && (!vowels.contains(v[(i + 1) as usize] as char)) {
                i += 1;
            }
            while i < j && (!vowels.contains(v[(j - 1) as usize] as char)) {
                j -= 1;
            }
            unsafe { std::ptr::swap(&mut v[i as usize], &mut v[j as usize]) }
            i += 1;
            j -= 1;
        }
        String::from_utf8(v).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::string::reverse_vowels::Solution;

    #[test]
    pub fn test() {
        // assert_eq!(Solution::reverse_vowels("".to_string()), "".to_string());
        //
        // assert_eq!(Solution::reverse_vowels("a.".to_string()), "a.".to_string());

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
