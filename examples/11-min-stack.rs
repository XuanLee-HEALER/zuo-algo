fn main() {}

struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if let Some(&min) = self.min.last() {
            if min > val {
                self.min.push(val)
            } else {
                self.min.push(min)
            }
        } else {
            self.min.push(val)
        }
        self.stack.push(val)
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}
