// 905. Sort Array By Parity

struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd): (Vec<i32>, Vec<i32>) = a.iter()
            .partition(|&x| *x % 2 == 0);
        even.append(&mut odd);
        even
    }

    pub fn sort_array_by_parity_with_and(a: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd): (Vec<i32>, Vec<i32>) = a.iter()
            .partition(|&x| *x & 1 == 0);
        even.append(&mut odd);
        even
    }
}


#[cfg(test)]
mod test {
    use super::Solution;
    use test::Bencher;

    #[test]
    fn test_sort_array_by_parity() {
        assert_eq!(Solution::sort_array_by_parity(vec![3, 1, 2, 4]), vec![2, 4, 3, 1]);
    }

    #[test]
    fn test_sort_array_by_parity_with_and() {
        assert_eq!(Solution::sort_array_by_parity_with_and(vec![3, 1, 2, 4]), vec![2, 4, 3, 1]);
    }

    #[bench]
    fn bench_sort_array_by_parity(b: &mut Bencher) {
        b.iter(move || {
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]);
        });
    }

    #[bench]
    fn bench_sort_array_by_parity_with_bitwise_and(b: &mut Bencher) {
        b.iter(move || {
            Solution::sort_array_by_parity_with_and(vec![3, 1, 2, 4]);
        });
    }
}