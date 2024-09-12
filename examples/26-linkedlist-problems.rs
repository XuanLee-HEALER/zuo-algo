fn main() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));

    let mut res = Solution::reverse_k_group(head, 3);
    while let Some(node) = res {
        println!("{}", node.val);
        res = node.next;
    }
}

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

struct Solution;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut sentinel = ListNode::new(-1);
        let mut flag_pos = &mut sentinel;

        loop {
            if Self::peek_head(&head, k) {
                for _ in 0..k {
                    if let Some(mut node) = head {
                        head = node.next.take();
                        node.next = flag_pos.next.take();
                        flag_pos.next = Some(node);
                    }
                }

                while let Some(ref mut next) = flag_pos.next {
                    flag_pos = next
                }
            } else {
                flag_pos.next = head;
                break;
            }
        }

        sentinel.next.take()
    }

    fn peek_head(mut head: &Option<Box<ListNode>>, k: i32) -> bool {
        let mut counter = 0;
        while let Some(ref node) = head {
            counter += 1;
            head = &node.next;
            if counter >= k {
                return true;
            }
        }

        false
    }
}
