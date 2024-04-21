struct Queue {
    queue: Vec<i32>,
}

impl Queue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, x: i32) {
        let n = self.queue.len();
        self.queue.insert(n, x);
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        self.queue.pop()
    }
}

fn main() {
    let mut q = Queue::new();

    // Test enqueue
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    assert_eq!(q.queue, vec![1, 2, 3]);

    // Test dequeue
    assert_eq!(q.dequeue(), Some(3));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.dequeue(), None);
}
