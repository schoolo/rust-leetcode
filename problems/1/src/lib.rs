pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0];
            }
            return vec![];
        }
        for x in 0..nums.len() {
            for y in (x + 1)..nums.len() {
                if nums[x] + nums[y] == target {
                    return vec![x as i32, y as i32];
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let source = vec![];
        let res = Solution::two_sum(source, 3);
        assert_eq!(res, vec![]);
    }
    #[test]
    fn one_elem() {
        let source = vec![1];
        let res = Solution::two_sum(source, 1);
        assert_eq!(res, vec![0]);
    }
    #[test]
    fn two_elem_with_solution() {
        let source = vec![1,2];
        let res = Solution::two_sum(source, 3);
        assert_eq!(res, vec![0,1]);
    }
    #[test]
    fn two_elem_no_solution() {
        let source = vec![1,2];
        let res = Solution::two_sum(source, 4);
        assert_eq!(res, vec![]);
    }
    #[test]
    fn many_elems_with_solution() {
        let source = vec![4,6, 133, 222, 50];
        let res = Solution::two_sum(source, 355);
        assert_eq!(res, vec![2,3]);
    }
    #[test]
    fn many_elems_no_solution() {
        let source = vec![4,6, 133, 222, 50];
        let res = Solution::two_sum(source, 5);
        assert_eq!(res, vec![]);
    }
}
