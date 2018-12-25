use std::collections::HashMap;
use std::collections::HashSet;

/// 804. Unique Morse Code Words

struct Solution1;

struct Solution2;


impl Solution1 {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse_codes = Solution1::build_morse_codes();
        let mut unique_morse_codes: HashSet<String> = HashSet::new();
        for word in words {
            let mut transformed_morse_code = String::new();
            for c in word.chars() {
                if let Some(code) = morse_codes.get(&c) {
                    transformed_morse_code.push_str(code);
                }
            }
            unique_morse_codes.insert(transformed_morse_code);
        }
        unique_morse_codes.len() as i32
    }

    pub fn build_morse_codes<'a>() -> HashMap<char, &'a str> {
        let mut morse_codes: HashMap<char, &str> = HashMap::new();
        // ".-","-...","-.-.","-..", ".","..-.","--.","....","..",".---","-.-",".-..","--","-.",
        // "---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."
        morse_codes.insert('a', ".-");
        morse_codes.insert('b', "-...");
        morse_codes.insert('c', "-.-.");
        morse_codes.insert('d', "-..");
        morse_codes.insert('e', ".");
        morse_codes.insert('f', "..-.");
        morse_codes.insert('g', "--.");
        morse_codes.insert('h', "....");
        morse_codes.insert('i', "..");
        morse_codes.insert('j', ".---");
        morse_codes.insert('k', "-.-");
        morse_codes.insert('l', ".-..");
        morse_codes.insert('m', "--");
        morse_codes.insert('n', "-.");
        morse_codes.insert('o', "---");
        morse_codes.insert('p', ".--.");
        morse_codes.insert('q', "--.-");
        morse_codes.insert('r', ".-.");
        morse_codes.insert('s', "...");
        morse_codes.insert('t', "-");
        morse_codes.insert('u', "..-");
        morse_codes.insert('v', "...-");
        morse_codes.insert('w', ".--");
        morse_codes.insert('x', "-..-");
        morse_codes.insert('y', "-.--");
        morse_codes.insert('z', "--..");
        morse_codes
    }
}

// better solution
impl Solution2 {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        // a - z
        let morse_codes = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..",
            ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-",
            "...-", ".--", "-..-", "-.--", "--.."];
        let mut unique_morse_codes: HashSet<String> = HashSet::new();
        for word in words {
            let mut transformed_morse_code = String::new();
            for c in word.chars() {
                if let Some(code) = morse_codes.get(c as usize - 'a' as usize) {
                    transformed_morse_code.push_str(code);
                }
            }
            unique_morse_codes.insert(transformed_morse_code);
        }
        unique_morse_codes.len() as i32
    }
}


#[cfg(test)]
mod test {
    use super::{Solution1, Solution2};
    use test::Bencher;

    #[test]
    fn test_unique_morse_representations() {
        let test_words = vec!["gin".to_string(),
                              "zen".to_string(),
                              "gig".to_string(),
                              "msg".to_string()];
        assert_eq!(Solution1::unique_morse_representations(test_words), 2);
    }

    #[bench]
    fn bench_unique_morse_representations_hash_map(b: &mut Bencher) {
        b.iter(move || {
            let test_words = vec!["gin".to_string(),
                                  "zen".to_string(),
                                  "gig".to_string(),
                                  "msg".to_string()];
            Solution1::unique_morse_representations(test_words);
        });
    }

    #[test]
    fn test_unique_morse_representations_2() {
        let test_words = vec!["gin".to_string(),
                              "zen".to_string(),
                              "gig".to_string(),
                              "msg".to_string()];
        assert_eq!(Solution2::unique_morse_representations(test_words), 2);
    }

    #[bench]
    fn bench_unique_morse_representations_array(b: &mut Bencher) {
        b.iter(move || {
            let test_words = vec!["gin".to_string(),
                                  "zen".to_string(),
                                  "gig".to_string(),
                                  "msg".to_string()];
            Solution2::unique_morse_representations(test_words);
        });
    }


}