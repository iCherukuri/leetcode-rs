/// 771. Jewels and stones (easy)

struct Solution;

impl Solution {
    fn num_jewels_in_stones_1(j: String, s: String) -> i32 {
        let mut count = 0;
        for c in s.chars() {
            if j.contains(c) {
                count += 1;
            }
        }
        count
    }

    fn num_jewels_in_stones_2(j: String, s: String) -> i32 {
        let mut count = 0;
        for c in s.chars() {
            if let Some(_) = j.find(c) {
                count += 1;
            }
        }
        count
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_jewels_in_stones() {
        assert_eq!(Solution::num_jewels_in_stones_1(String::from("aA"), String::from("aAAbbbb")), 3);
        assert_eq!(Solution::num_jewels_in_stones_1(String::from("z"), String::from("ZZ")), 0);

        assert_eq!(Solution::num_jewels_in_stones_2(String::from("aA"), String::from("aAAbbbb")), 3);
        assert_eq!(Solution::num_jewels_in_stones_2(String::from("z"), String::from("ZZ")), 0);
    }
}
