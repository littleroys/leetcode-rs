// #43 multiply strings
struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut iter1 = num1.chars().rev();
        let mut iter1 = num1.chars().rev();

        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::string::multiply_str::Solution;

    #[test]
    pub fn test_multiply() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088"
        )
    }
}
