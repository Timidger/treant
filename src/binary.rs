use std::ptr::{Unique, null_mut};
//use std::cell::UnsafeCell;
struct BinaryTree<T> {
    root: Node<T>
}

struct Node<T> {
    parent: *mut Node<T>,
    children: (Option<Unique<Node<T>>>, Option<Unique<Node<T>>>),
    value: T
}

impl <T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            parent: null_mut(),
            children: (None, None),
            value: value
        }
    }
}

impl <T> BinaryTree<T> {
    fn new(value: T) -> Self {
        BinaryTree {
            root: Node::new(value)
        }
    }
}
