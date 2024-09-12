use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let node1 = Rc::new(RefCell::new(DoubleListNode::new(1)));
    let node2 = Rc::new(RefCell::new(DoubleListNode::new(2)));
    let node3 = Rc::new(RefCell::new(DoubleListNode::new(3)));
    let node4 = Rc::new(RefCell::new(DoubleListNode::new(4)));
    let node5 = Rc::new(RefCell::new(DoubleListNode::new(5)));
    node1.borrow_mut().next = Some(Rc::clone(&node2));
    node2.borrow_mut().next = Some(Rc::clone(&node3));
    node3.borrow_mut().next = Some(Rc::clone(&node4));
    node4.borrow_mut().next = Some(Rc::clone(&node5));
    node2.borrow_mut().prev = Some(Rc::clone(&node1));
    node3.borrow_mut().prev = Some(Rc::clone(&node2));
    node4.borrow_mut().prev = Some(Rc::clone(&node3));
    node5.borrow_mut().prev = Some(Rc::clone(&node4));

    // iterate_double_list(Some(node1));
    let rev_head = reverse_double_list(Some(node1));
    iterate_double_list(rev_head);
}

fn iterate_double_list(head: Option<Rc<RefCell<DoubleListNode>>>) {
    let mut head = head;
    while let Some(node) = head {
        println!("=> node value {}", node.borrow().val);
        head = node.borrow_mut().next.take();
    }
}

fn reverse_double_list(
    head: Option<Rc<RefCell<DoubleListNode>>>,
) -> Option<Rc<RefCell<DoubleListNode>>> {
    let mut result = None;
    let mut head = head;

    while let Some(node) = head {
        let next = node.borrow_mut().next.take();
        node.borrow_mut().prev = next.clone();
        node.borrow_mut().next = result;
        head = next;
        result = Some(node);
    }

    result
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

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DoubleListNode {
    pub val: i32,
    pub prev: Option<Rc<RefCell<DoubleListNode>>>,
    pub next: Option<Rc<RefCell<DoubleListNode>>>,
}

impl DoubleListNode {
    #[inline]
    fn new(val: i32) -> Self {
        DoubleListNode {
            prev: None,
            next: None,
            val,
        }
    }
}

pub fn reverse_list_sample(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut prev = None;

    while let Some(mut head_node) = head {
        let next = head_node.next.take();
        head_node.next = prev;
        prev = Some(head_node);
        head = next;
    }

    prev
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut prev = None;

    while head.is_some() {
        let mut head_node = head.unwrap();
        let next = head_node.next.clone();
        head_node.next = prev.clone();
        prev = Some(head_node.clone());
        head = next;
    }

    prev
}
