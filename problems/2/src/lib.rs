// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}
impl Solution {
    fn _add_two_numbers(
        l1: &Option<Box<ListNode>>,
        l2: &Option<Box<ListNode>>,
        rem: i32,
    ) -> Option<Box<ListNode>> {
        let mut current: Option<Box<ListNode>>;
        match (l1, l2) {
            (None, None) => {
                if rem != 0 {
                    current = Some(Box::new(ListNode::new(rem)));
                    current.as_mut().unwrap().next = None;
                } else {
                    current = None;
                }
            },
            (None, Some(x)) => {
                let val = x.val + rem;
                let remainder: i32;
                match val {
                    0..=9 => {
                        current = Some(Box::new(ListNode::new(val)));
                        remainder = 0;
                    }
                    _ => {
                        current = Some(Box::new(ListNode::new(val % 10)));
                        remainder = 1;
                    }
                }
                current.as_mut().unwrap().next =
                    Solution::_add_two_numbers(&None, &x.next, remainder);
            },
            (Some(x), None) => {
                let val = x.val + rem;
                let remainder: i32;
                match val {
                    0..=9 => {
                        current = Some(Box::new(ListNode::new(val)));
                        remainder = 0;
                    }
                    _ => {
                        current = Some(Box::new(ListNode::new(val % 10)));
                        remainder = 1;
                    }
                }
                current.as_mut().unwrap().next =
                    Solution::_add_two_numbers(&x.next, &None, remainder);
            },
            (Some(x), Some(y)) => {
                let val = x.val + y.val + rem;
                let remainder: i32;
                match val {
                    0..=9 => {
                        current = Some(Box::new(ListNode::new(val)));
                        remainder = 0;
                    }
                    _ => {
                        current = Some(Box::new(ListNode::new(val % 10)));
                        remainder = 1;
                    }
                }
                current.as_mut().unwrap().next =
                    Solution::_add_two_numbers(&x.next, &y.next, remainder);
            }
        }
        return current;
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return Solution::_add_two_numbers(&l1, &l2, 0);
    }
}

fn vec_to_linked_list(truth: &[i32]) -> Option<Box<ListNode>> {
    if truth.is_empty() {
        return None;
    }

    let mut head = Box::new(ListNode::new(truth[0]));
    if truth.len() == 1 {
        head.next = None;
    } else {
        head.next = vec_to_linked_list(&truth[1..]);
    }
    return Some(head);
}
fn verify_linked_list(truth: &[i32], llist: &Option<Box<ListNode>>) -> bool {
    if truth.is_empty() {
        return !llist.is_some();
    }
    if !llist.is_some() {
        return truth.is_empty();
    }

    let node = llist.as_ref().unwrap();
    return truth[0] == node.val && verify_linked_list(&truth[1..], &node.next);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_linked_list_zero_elem() {
        assert!(verify_linked_list(&vec![], &None));
    }
    #[test]
    fn test_verify_linked_list_one_elem() {
        assert!(verify_linked_list(
            &vec![1],
            &Some(Box::new(ListNode::new(1)))
        ));
    }
    #[test]
    fn test_vec_to_linked_list() {
        assert!(verify_linked_list(
            &vec![1, 2, 3, 4, 5],
            &vec_to_linked_list(&vec![1, 2, 3, 4, 5])
        ));
    }
    #[test]
    fn test_single_digit_add() {
        let first = vec_to_linked_list(&vec![1]);
        let second = vec_to_linked_list(&vec![2]);
        let expected = vec_to_linked_list(&vec![3]);
        let result = Solution::add_two_numbers(first, second);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_add() {
        let first = vec_to_linked_list(&vec![0, 5, 2, 1]);
        let second = vec_to_linked_list(&vec![0, 5, 2]);
        let expected = vec_to_linked_list(&vec![0, 0, 5, 1]);
        let result = Solution::add_two_numbers(first, second);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_add_overflow() {
        let first = vec_to_linked_list(&vec![9, 9, 9]);
        let second = vec_to_linked_list(&vec![1]);
        let expected = vec_to_linked_list(&vec![0, 0, 0, 1]);
        let result = Solution::add_two_numbers(first, second);
        assert_eq!(expected, result);
    }
}
