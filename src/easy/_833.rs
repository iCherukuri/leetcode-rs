// 832. Flipping an Image

struct Solution;

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        for mut inner_vec in a {
            inner_vec.reverse();
            let new_vec: Vec<i32> = inner_vec.into_iter().map(|x| x ^ 1).collect();
            result.push(new_vec);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test_flip_and_invert_image() {
        let test_data = vec![vec![1,0,0,1]];
        assert_eq!(Solution::flip_and_invert_image(test_data), vec![vec![0,1,1,0]]);
    }
}