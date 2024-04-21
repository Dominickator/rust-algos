struct Stack {
    stack: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }
}

fn main() {
    let mut my_stack = Stack::new();

    // Test push
    my_stack.push(1);
    my_stack.push(2);
    my_stack.push(3);

    // Test pop
    assert_eq!(my_stack.pop(), Some(3));
    assert_eq!(my_stack.pop(), Some(2));
    assert_eq!(my_stack.pop(), Some(1));
    assert_eq!(my_stack.pop(), None);
}
