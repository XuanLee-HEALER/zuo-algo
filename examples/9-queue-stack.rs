fn main() {
    let mut queue = MyCircularQueue::new(10);

    assert!(queue.en_queue(1));
    assert!(queue.en_queue(2));
    assert!(queue.en_queue(3));
    assert_eq!(queue.front(), 1);
    assert_eq!(queue.rear(), 3);

    assert!(queue.de_queue());
    assert_eq!(queue.front(), 2);
    assert!(queue.de_queue());
    assert_eq!(queue.front(), 3);
    assert!(queue.de_queue());
    assert_eq!(queue.front(), -1);

    assert!(!queue.de_queue());
}

struct MyCircularQueue {
    l: usize,
    r: usize,
    size: usize,
    limit: usize,
    arr: [i32; 1000],
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            l: 0,
            r: 0,
            size: 0,
            limit: k as usize,
            arr: [0; 1000],
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.arr[self.r] = value;
            self.r = (self.r + 1) % self.limit;
            self.size += 1;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.l = (self.l + 1) % self.limit;
            self.size -= 1;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.arr[self.l]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let lst_ele_idx = if self.r == 0 {
                self.limit - 1
            } else {
                self.r - 1
            };
            self.arr[lst_ele_idx]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.limit
    }
}
