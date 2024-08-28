fn main() {}

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

pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut small_head = ListNode::new(-1);
    let mut big_head = ListNode::new(-1);

    let mut ref_small_head = &mut small_head;
    let mut ref_big_head = &mut big_head;

    while let Some(node) = head {
        if node.val < x {
            ref_small_head.next = Some(Box::new(ListNode::new(node.val)));
            ref_small_head = ref_small_head.next.as_mut().unwrap();
        } else {
            ref_big_head.next = Some(Box::new(ListNode::new(node.val)));
            ref_big_head = ref_big_head.next.as_mut().unwrap();
        }

        head = node.next;
    }

    ref_small_head.next = big_head.next;

    small_head.next
}
