enum State {
    Zig,
    Zag
}

struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut row: i32 = 0;

        let mut state = State::Zig;
        let mut rows = vec![String::new(); num_rows as usize];
        for c in s.chars() {
            match state {
                State::Zig => {
                    rows[row as usize].push(c);
                    if row == num_rows - 1 {
                        state = State::Zag;
                        row -= 1;
                        continue;
                    }
                    row += 1;
                }
                State::Zag => {
                    rows[row as usize].push(c);
                    if row == 0 {
                        state = State::Zig;
                        row += 1;
                        continue;
                    }
                    row -= 1;
                }
            }
        }

        rows.iter().map(|x| x.chars()).flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
    }
}
