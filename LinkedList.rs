use std::cell::RefCell;
use std::rc::Rc;

// Define a node of the linked list
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

// Define the linked list
struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Add an element at the front of the list
    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.clone(),
        }));
        self.head = Some(new_node);
    }

    // Pop an element from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.borrow_mut().next.clone();
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    while let Some(data) = list.pop_front() {
        println!("{}", data);
    }
}
