use std::borrow::Borrow;

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

pub struct Solution {}

impl Solution {
    pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
        let mut root = &head;
        let mut root2 = &head;
        for _ in 0..k {
            match root {
                None => {
                    break;
                }
                Some(node) => {
                    root = node.next.borrow();
                }
            }
        }
        loop {
            match root {
                None => {
                    break;
                }
                Some(node) => {
                    root = node.next.borrow();
                    match root2 {
                        None => {
                            break
                        },
                        Some(n) => {
                            root2 = n.next.borrow()
                        }
                    }
                }
            };
        }
        return match root2 {
            None => {
                0
            }
            Some(node) => {
                node.val
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    #[test]
    fn it_works() {
        let mut b = ListNode::new(2);
        b.next = Some(Box::new(ListNode::new(5)));
        let h = Some(Box::new(b));
        println!("{:?}", h);

        let s = Solution::kth_to_last(h, 2);
        println!("{}", s)
    }
}