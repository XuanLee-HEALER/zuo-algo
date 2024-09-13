use std::{cell::RefCell, rc::Rc};

fn main() {}

struct DoubleNode {
    val: i32,
    prev: Option<Rc<RefCell<DoubleNode>>>,
    next: Option<Rc<RefCell<DoubleNode>>>,
}

impl PartialEq for DoubleNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
            && if self.prev.is_none() && other.prev.is_none() {
                true
            } else if self.prev.is_some() && other.prev.is_some() {
                Rc::ptr_eq(self.prev.as_ref().unwrap(), other.prev.as_ref().unwrap())
            } else {
                false
            }
            && if self.next.is_none() && other.next.is_none() {
                true
            } else if self.next.is_some() && other.next.is_some() {
                Rc::ptr_eq(self.next.as_ref().unwrap(), other.next.as_ref().unwrap())
            } else {
                false
            }
    }
}

struct DoubleList<'a> {
    head: Option<&'a DoubleNode>,
    tail: Option<&'a DoubleNode>,
}

impl<'a> DoubleList<'a> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn move_node_to_tail(&mut self, node: &'a mut DoubleNode) {
        if node != self.tail.unwrap() {
            let nxt_node = node.next.take().unwrap();
            let prv_node = node.prev.take();
            node.next = None;
            if let Some(prv) = prv_node {
                nxt_node.borrow_mut().prev = Some(Rc::clone(&prv));
                prv.borrow_mut().next = Some(Rc::clone(&nxt_node));
            } else {
                self.head = Some(node);
            }
            self.tail = Some(node);
        }
    }
}
