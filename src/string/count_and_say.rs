// #38 count and say
struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = String::from("1");
        for _ in 1..n {
            result = Solution::sub(result.as_str());
        }
        result
    }

    pub fn sub(input: &str) -> String {
        let (mut input, mut result) = (input, String::new());

        while input[..].len() > 0 {
            let flag: char = input.chars().nth(0).unwrap();
            let counter = input.chars().take_while(|ch| ch.eq(&flag)).count();
            input = &input[counter..];
            result.push_str(&counter.to_string());
            result.push(flag);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::string::count_and_say::Solution;

    #[test]
    pub fn test_count_1() {
        assert_eq!(Solution::count_and_say(1), "1".to_string())
    }

    #[test]
    pub fn test_count_2() {
        assert_eq!(Solution::count_and_say(2), "11".to_string())
    }

    #[test]
    pub fn test_count_3() {
        assert_eq!(Solution::count_and_say(3), "21".to_string())
    }

    #[test]
    pub fn test_count_4() {
        assert_eq!(Solution::count_and_say(4), "1211".to_string())
    }

    #[test]
    pub fn test_count_5() {
        assert_eq!(Solution::count_and_say(5), "111221".to_string())
    }
}
