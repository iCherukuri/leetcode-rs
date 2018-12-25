/// 929. Unique Email Addresses (easy)
use std::collections::hash_set::HashSet;

struct Solution;

impl Solution {
    fn num_unique_emails(emails: Vec<String>) -> i32 {
        // stores unique elements
        let mut hash_set: HashSet<String> = HashSet::new();
        for email in emails {
            // split email address as local name and domain
            let result: Vec<&str> = email.split('@').collect();
            let local_name = result[0];
            let domain = result[1];
            // test.email+alex => [test.email, alex]
            let local_name_split: Vec<&str> = local_name.split('+').collect();
            let mut local_name_to_store = local_name_split[0].to_string().to_lowercase();
            // retain all chars except '.'
            local_name_to_store.retain(|c| c != '.');
            hash_set.insert(local_name_to_store + "@" + domain);
        }
        hash_set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_unique_emails() {
        let test_emails = vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        ];
        assert_eq!(Solution::num_unique_emails(test_emails), 2);
    }
}