use std::collections::VecDeque;

fn main() {}

struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.in_stack.push(x)
    }

    fn pop(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while let Some(v) = self.in_stack.pop() {
                self.out_stack.push(v);
            }
        }

        self.out_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while let Some(v) = self.in_stack.pop() {
                self.out_stack.push(v);
            }
        }

        *self.out_stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}

struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self { q: VecDeque::new() }
    }

    fn push(&mut self, x: i32) {
        let size = self.q.len();
        self.q.push_back(x);
        for _ in 0..size {
            let head_val = self.q.pop_front().unwrap();
            self.q.push_back(head_val);
        }
    }

    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}
