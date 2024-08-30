fn main() {}

struct MyCircularDeque {
    q: [i32; 1000],
    l: usize,
    r: usize,
    limit: usize,
    size: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            q: [0; 1000],
            l: 0,
            r: 0,
            limit: k as usize,
            size: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            if self.is_empty() {
                // 如果l和r都为0，删除元素索引不会回到0，因为初始值0实际上没有指向任何值，所以这里需要重置
                self.l = 0;
                self.r = 0;
                self.q[self.l] = value
            } else {
                self.l = if self.l == 0 {
                    self.limit - 1
                } else {
                    self.l - 1
                };
                self.q[self.l] = value
            }
            self.size += 1;
            true
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            if self.is_empty() {
                // 如果l和r都为0，删除元素索引不会回到0，因为初始值0实际上没有指向任何值，所以这里需要重置
                self.l = 0;
                self.r = 0;
                self.q[self.r] = value
            } else {
                self.r = if self.r == self.limit - 1 {
                    0
                } else {
                    self.r + 1
                };
                self.q[self.r] = value
            }
            self.size += 1;
            true
        }
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.l = if self.l == self.limit - 1 {
                0
            } else {
                self.l + 1
            };
            self.size -= 1;
            true
        }
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.r = if self.r == 0 {
                self.limit - 1
            } else {
                self.r - 1
            };
            self.size -= 1;
            true
        }
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.l]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.r]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.limit
    }
}

struct MyCircularDeque1 {
    k: usize,
    q: Vec<i32>,
    size: usize,
    front: usize,
    rear: usize,
}

impl MyCircularDeque1 {
    /**
     * Initialize your data structure here. Set the size of the deque to be k.
     */
    fn new(k: i32) -> Self {
        MyCircularDeque1 {
            k: k as usize,
            q: vec![0; k as usize],
            size: 0,
            front: 0,
            rear: (k as usize) - 1,
        }
    }

    /**
     * Adds an item at the front of Deque. Return true if the operation is successful.
     */
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front + self.k - 1) % self.k;
        self.q[self.front] = value;
        self.size += 1;
        true
    }

    /**
     * Adds an item at the rear of Deque. Return true if the operation is successful.
     */
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.rear = (self.rear + 1) % self.k;
        self.q[self.rear] = value;
        self.size += 1;
        true
    }

    /**
     * Deletes an item from the front of Deque. Return true if the operation is successful.
     */
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.k;
        self.size -= 1;
        true
    }

    /**
     * Deletes an item from the rear of Deque. Return true if the operation is successful.
     */
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear + self.k - 1) % self.k;
        self.size -= 1;
        true
    }

    /**
     * Get the front item from the deque.
     */
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.front]
        }
    }

    /**
     * Get the last item from the deque.
     */
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.rear]
        }
    }

    /**
     * Checks whether the circular deque is empty or not.
     */
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /**
     * Checks whether the circular deque is full or not.
     */
    fn is_full(&self) -> bool {
        self.size == self.k
    }
}
