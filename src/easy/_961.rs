struct Solution;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut count_arr = [0; 10000];
        let n = a.len() / 2;
        for i in a {
            count_arr[i as usize] += 1;
            if count_arr[i as usize] == n {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_repeated_n_times() {
        assert_eq!(Solution::repeated_n_times(vec![2, 4, 7, 2, 7, 2]), 2);
    }
}