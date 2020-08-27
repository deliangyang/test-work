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
        let mut root = head.clone();
        let mut s1 = 0;
        let mut root2 = head.clone();
        while s1 < k {
            s1 += 1;
            match root {
                None => {
                    break
                },
                Some(node) => {
                    root = node.next;
                }
            }
        }
        loop {
            match root {
                None => {
                    break;
                }
                Some(node) => {
                    root = node.next;
                    root2 = root2.unwrap().next
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

        let s = Solution::kth_to_last(h, 1);
        println!("{}", s)
    }
}