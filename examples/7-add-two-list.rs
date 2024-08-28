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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = ListNode::new(0);
    let mut ref_res = &mut res;

    let mut l1 = l1;
    let mut l2 = l2;

    let mut carry = 0;
    while let (Some(l1_node), Some(l2_node)) = (l1.take(), l2.take()) {
        let tmp = l1_node.val + l2_node.val + carry;
        let tmp_node_val = tmp % 10;
        carry = tmp / 10;

        ref_res.next = Some(Box::new(ListNode::new(tmp_node_val)));
        ref_res = ref_res.next.as_mut().unwrap();

        l1 = l1_node.next;
        l2 = l2_node.next;
        if l1.is_none() || l2.is_none() {
            break;
        }
    }

    while let Some(l1_node) = l1 {
        let tmp = l1_node.val + carry;
        let tmp_node_val = tmp % 10;
        carry = tmp / 10;

        ref_res.next = Some(Box::new(ListNode::new(tmp_node_val)));
        ref_res = ref_res.next.as_mut().unwrap();

        l1 = l1_node.next;
    }

    while let Some(l2_node) = l2 {
        let tmp = l2_node.val + carry;
        let tmp_node_val = tmp % 10;
        carry = tmp / 10;

        ref_res.next = Some(Box::new(ListNode::new(tmp_node_val)));
        ref_res = ref_res.next.as_mut().unwrap();

        l2 = l2_node.next;
    }

    if carry == 1 {
        ref_res.next = Some(Box::new(ListNode::new(carry)));
    }

    res.next
}

pub fn add_two_numbers1(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = ListNode::new(0);
    let mut ref_res = &mut res;

    let mut l1 = l1;
    let mut l2 = l2;

    let mut carry = 0;
    while l1.is_some() || l2.is_some() {
        let v1 = if let Some(l1_node) = l1 {
            l1 = l1_node.next;
            l1_node.val
        } else {
            0
        };

        let v2 = if let Some(l2_node) = l2 {
            l2 = l2_node.next;
            l2_node.val
        } else {
            0
        };

        let tmp = v1 + v2 + carry;
        let tmp_node_val = tmp % 10;
        carry = tmp / 10;

        ref_res.next = Some(Box::new(ListNode::new(tmp_node_val)));
        ref_res = ref_res.next.as_mut().unwrap();
    }

    if carry == 1 {
        ref_res.next = Some(Box::new(ListNode::new(carry)));
    }

    res.next
}

pub fn add_two_numbers_sample(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;
    let mut carry = 0;

    let mut l1 = l1;
    let mut l2 = l2;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        carry = sum / 10;
        let digit = sum % 10;

        current.next = Some(Box::new(ListNode::new(digit)));
        current = current.next.as_mut().unwrap();
    }

    dummy_head.next
}
