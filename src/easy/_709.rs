struct Solution;

impl Solution {
    fn to_lower_case(str: String) -> String {
        let mut lower_case_vec: Vec<u8> = Vec::with_capacity(str.len());
        for c in str.as_bytes() {
            let lower_case_char: u8 = match *c {
                65..=90 => *c + 32,
                _ => *c,
            };
            lower_case_vec.push(lower_case_char);
        }
        String::from_utf8(lower_case_vec).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_to_lower_case() {
        assert_eq!(Solution::to_lower_case(String::from("HELLO")), "hello");
    }
}