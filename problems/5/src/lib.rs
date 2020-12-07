pub struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        for len in (1..=s.len()).rev() {
            let subs = Solution::substrings_of_length(&s, len);
            for sub in subs {
                if Solution::is_palindrome(sub) {
                    return sub.to_string();
                }
            }
        }

        String::new()
    }

    fn substrings_of_length(s: &str, length: usize) -> Vec<&str> {
        let mut begin = 0 as usize;
        let mut end = length as usize;
        let mut result = vec![] as Vec<&str>;
        while end <= s.len() {
            result.push(&s[begin..end]);
            begin += 1;
            end += 1;
        }

        result
    }

    fn is_palindrome(s: &str) -> bool {
        let mut forward = s.char_indices();
        let mut backward = s.char_indices().rev();
        loop {
            let f = forward.next();
            let b = backward.next();
            match (f, b) {
                (Some(_), None) => break,
                (None, Some(_)) => break,
                (None, None) => break,
                (Some(x), Some(y)) => {
                    let (fidx, fc) = x;
                    let (bidx, bc) = y;
                    if bidx <= fidx {
                        break;
                    }
                    if fc != bc {
                        return false;
                    }
                }
            }
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_substrings_of_length() {
        assert_eq!(
            vec!["a", "b", "c"],
            Solution::substrings_of_length(&"abc", 1)
        );
        assert_eq!(vec!["ab", "bc"], Solution::substrings_of_length(&"abc", 2));
        assert_eq!(vec!["abc"], Solution::substrings_of_length(&"abc", 3));
    }
    #[test]
    fn test_is_palindrome() {
        assert_eq!(true, Solution::is_palindrome(&"a"));
        assert_eq!(true, Solution::is_palindrome(&"bb"));
        assert_eq!(true, Solution::is_palindrome(&"aba"));
        assert_eq!(true, Solution::is_palindrome(&"abaaba"));
        assert_eq!(false, Solution::is_palindrome(&"ab"));
        assert_eq!(false, Solution::is_palindrome(&"abac"));
    }
    #[test]
    fn test_longest_palindrome_empty() {
        assert_eq!(&"", &Solution::longest_palindrome("".to_string()));
    }
    #[test]
    fn test_longest_palindrome_single_char() {
        assert_eq!(&"a", &Solution::longest_palindrome("a".to_string()));
    }
    #[test]
    fn test_longest_palindrome_double_char() {
        assert_eq!(&"bb", &Solution::longest_palindrome("bb".to_string()));
    }
    #[test]
    fn test_longest_palindrome_multiple_chars() {
        assert_eq!(
            &"aabbaa",
            &Solution::longest_palindrome("abaaabbaa".to_string())
        );
    }
}
