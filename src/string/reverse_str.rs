// #344 reverse string
struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

#[cfg(test)]
mod test {
    use crate::string::reverse_str::Solution;

    #[test]
    pub fn test() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        println!("{:?}", s);
    }
}
