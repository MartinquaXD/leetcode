//https://leetcode.com/problems/add-two-numbers/


use std::ops::{Rem, RemAssign};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution();

impl Solution {
//    fn from_list(l: Option<Box<ListNode>>) -> i32 {
//        let mut val = 0;
//        let mut mul = 1;
//        let mut tmp = &l;
//        while let Some(n) = tmp {
//            val += n.val * mul;
//            mul *= 10;
//            tmp = &n.next;
//        };
//        val
//    }
//
//    fn to_list(mut v: i32) -> Option<Box<ListNode>> {
//        let mut rest = None;
//
//        while v > 0 {
//            let digit = v % 10;
//
//            let current_digit = Box::new(ListNode::new(digit));
//
//            rest = rest.map(|rest: Option<Box<ListNode>>| {
//                if let Some(rest) = rest {
//                    rest.next = Some(current_digit);
//                    rest
//                } else {
//                    current_digit
//                }
//            });
//
//            v /= 10;
//        };
//
//        rest
//    }
//
//    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//        let sum = Solution::from_list(l1) + Solution::from_list(l2);
//        Solution::to_list(sum)
//    }
}
