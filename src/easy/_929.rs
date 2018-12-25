/// 929. Unique Email Addresses (easy)
use std::collections::hash_set::HashSet;

struct Solution;

impl Solution {
    fn num_unique_emails_1(emails: Vec<String>) -> i32 {
        // stores unique elements
        let mut unique_emails: HashSet<String> = HashSet::new();
        for email in emails {
            let mut unique_email = String::new();
            if let Some(at_symbol_position) = email.find('@') {
                let local_name = &email[0..at_symbol_position];
                let domain_str = &email[at_symbol_position..];
                for c in local_name.chars() {
                    if c == '.' {
                        continue
                    } else if c == '+' {
                        break
                    } else {
                        unique_email.push(c);
                    }
                }
                unique_email += domain_str; // append domain
                unique_emails.insert(unique_email);
            }
        }
        unique_emails.len() as i32
    }

    // less efficient version than above
    fn num_unique_emails_2(emails: Vec<String>) -> i32 {
        // stores unique elements
        let mut unique_emails: HashSet<String> = HashSet::new();
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
            unique_emails.insert(local_name_to_store + "@" + domain);
        }
        unique_emails.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_unique_emails() {
        let test_emails1 = vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        ];

        let test_emails2 = vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        ];
        assert_eq!(Solution::num_unique_emails_1(test_emails1), 2);
        assert_eq!(Solution::num_unique_emails_2(test_emails2), 2);
    }
}