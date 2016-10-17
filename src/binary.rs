use std::ptr::null_mut;
use std::cell::UnsafeCell;

/// A tree where each node has 0, 1, or 2 children.
struct BinaryTree<T> {
    root: UnsafeCell<BinaryNode<T>>
}

/// A node that has 0, 1, or 2 children and maybe a parent.
/// If the node has no parent, it is the root of the tree.
struct BinaryNode<T> {
    parent: *mut BinaryNode<T>,
    children: (Option<Box<BinaryNode<T>>>, Option<Box<BinaryNode<T>>>),
    value: T
}

struct BinaryView<T>(*mut BinaryNode<T>);
struct BinaryViewMut<T>(*mut BinaryNode<T>);

impl <T> BinaryNode<T> {
    /// Constructs a new node for a binary tree.
    fn new(value: T) -> Self {
        BinaryNode {
            parent: null_mut(),
            children: (None, None),
            value: value
        }
    }
}

impl <T> BinaryTree<T> {
    /// Constructs a new binary tree.
    pub fn new(value: T) -> Self {
        BinaryTree {
            root: UnsafeCell::new(BinaryNode::new(value))
        }
    }

    /// Constructs a view into the binary tree. 
    /// You can have as many of these as you want.
    pub fn view(&self) -> BinaryView<T> {
        BinaryView(self.root.get() as *mut _)
    }

    /// Constructs a mutable view into the binary tree.
    /// You can only have one of these at a time, and you can not have
    /// any immutable views active alongside them mutable view.
    pub fn view_mut(&mut self) -> BinaryViewMut<T> {
        BinaryViewMut(self.root.get() as *mut _)
    }
}
