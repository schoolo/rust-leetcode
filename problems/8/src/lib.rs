struct Solution {}
impl Solution {
    fn strip(s: &str) -> &str {
        let i = s.find(|c| c != ' ');
        match i {
            Some(x) => return &s[x..],
            None => return &"",
        }
    }

    fn get_multiplier(s: &str) -> (i32, &str) {
        match s.chars().next() {
            Some('-') => return (-1, &s[1..]),
            Some('+') => return (1, &s[1..]),
            _ => return (1, &s[..]),
        }
    }

    fn atoi(s: &str, mul: i32) -> i32 {
        let mut res: i32 = 0;
        let max = if mul == 1 {
            i32::max_value()
        } else {
            i32::min_value()
        };
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    let mut maybe_res = res.checked_mul(10);
                    match maybe_res {
                        Some(x) => {
                            maybe_res = x.checked_add(c.to_digit(10).unwrap() as i32);
                            match maybe_res {
                                Some(x) => res = x,
                                None => return max,
                            }
                        }
                        None => return max,
                    }
                }
                _ => break,
            }
        }

        res * mul
    }

    pub fn my_atoi(s: String) -> i32 {
        let (mul, num) = Solution::get_multiplier(Solution::strip(&s));

        Solution::atoi(num, mul)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        assert_eq!(1, Solution::my_atoi("1".to_string()));
    }
    #[test]
    fn test_large() {
        assert_eq!(1234, Solution::my_atoi("1234".to_string()));
    }
    #[test]
    fn test_negative() {
        assert_eq!(-1, Solution::my_atoi("-1".to_string()));
    }
    #[test]
    fn test_negative_large() {
        assert_eq!(-1234, Solution::my_atoi("-1234".to_string()));
    }
    #[test]
    fn test_only_minus() {
        assert_eq!(0, Solution::my_atoi("-".to_string()));
    }
    #[test]
    fn test_only_plus() {
        assert_eq!(0, Solution::my_atoi("+".to_string()));
    }
    #[test]
    fn test_only_whitespace() {
        assert_eq!(0, Solution::my_atoi(" ".to_string()));
        assert_eq!(0, Solution::my_atoi("    ".to_string()));
    }
    #[test]
    fn test_many_non_numeric() {
        assert_eq!(0, Solution::my_atoi("!@##@$@#$%".to_string()));
    }
    #[test]
    fn test_number_then_not() {
        assert_eq!(2343, Solution::my_atoi(" 2343@#$#@".to_string()));
    }
    #[test]
    fn test_negative_overflow() {
        assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_string()))
    }
}
