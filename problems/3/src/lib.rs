use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut next = &s as &str;
        let mut longest_count = 0;
        loop {
            let (res, after) = Solution::nondupe_substr(next);
            if res.len() > longest_count {
                longest_count = res.len();
            }
            if next.len() == 0 {
                break;
            }
            next = &(*after);
        }
        return longest_count as i32;
    }

    fn nondupe_substr(s: &str) -> (&str, &str) {
        let mut memo = HashSet::new();
        let mut count = 0;
        for c in s.chars() {
            let is_new = memo.insert(c);
            println!("{}", c);
            if !is_new {
                break;
            }
            count += 1;
        }

        if count == s.len() {
            return (&s[..count], &"" as &str);
        }

        (&s[..count], &s[count..])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nondupe_str() {
        let source = &"abcdaefg";
        let (res, after) = Solution::nondupe_substr(source);
        assert_eq!(res, "abcd");
        assert_eq!(after, "aefg");

        let (res1, after1) = Solution::nondupe_substr(after);
        assert_eq!(res1, "aefg");
        assert_eq!(after1, "");
    }

    #[test]
    fn test_longest_at_beginning() {
        let source = "abcda";
        assert_eq!(Solution::length_of_longest_substring(source.to_string()), 4);
    }

    #[test]
    fn test_longest_at_end() {
        let source = "abcdabcdefg";
        assert_eq!(Solution::length_of_longest_substring(source.to_string()), 7);
    }

    #[test]
    fn test_longest_in_middle() {
        let source = "abcdabcdefgghje";
        assert_eq!(Solution::length_of_longest_substring(source.to_string()), 7);
    }
}
