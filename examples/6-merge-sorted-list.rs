fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    }));
    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let merge_res = merge_two_lists(list1, list2);
    iterate_list(merge_res);
}

fn iterate_list(head: Option<Box<ListNode>>) {
    let mut head = head;
    while let Some(node) = head {
        println!("=> {}", node.val);
        head = node.next;
    }
}

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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    } else if list2.is_none() {
        return list1;
    }

    let mut dummy = ListNode::new(0);
    let mut tail = &mut dummy;

    let mut l1 = list1;
    let mut l2 = list2;

    while let (Some(mut node1), Some(mut node2)) = (l1.take(), l2.take()) {
        if node1.val < node2.val {
            l1 = node1.next.take();
            l2 = Some(node2);
            tail.next = Some(node1);
        } else {
            l2 = node2.next.take();
            l1 = Some(node1);
            tail.next = Some(node2);
        }
        tail = tail.next.as_mut().unwrap();
        if l1.is_none() || l2.is_none() {
            break;
        }
    }

    tail.next = if l1.is_some() { l1 } else { l2 };

    dummy.next
}

pub fn merge_two_lists_sample1(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut r = &mut l1;
    while l2.is_some() {
        if r.is_none() || l2.as_ref()?.val < r.as_ref()?.val {
            std::mem::swap(r, &mut l2);
        }
        r = &mut r.as_mut()?.next;
    }
    l1
}
