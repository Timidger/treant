use super::view::{BinaryView, BinaryViewMut, BinaryViewInner};
use std::ptr::null_mut;
use std::cell::UnsafeCell;
use std::marker::PhantomData;


/// A tree where each node has 0, 1, or 2 children.
pub struct BinaryTree<T> {
    root: UnsafeCell<BinaryNode<T>>
}

/// A node that has 0, 1, or 2 children and maybe a parent.
/// If the node has no parent, it is the root of the tree.
pub struct BinaryNode<T> {
    parent: *mut BinaryNode<T>,
    children: (Option<Box<BinaryNode<T>>>, Option<Box<BinaryNode<T>>>),
    value: T
}

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

    /// Gets the inner `UnsafeCell` that backs the tree.
    ///
    /// # Safety
    /// Returns a reference to the underlying `UnsafeCell`.
    /// This can be used to circumvent `BinaryTree`'s safety checks.
    /// This function is unsafe because `UnsafeCell`'s field is public.
    pub unsafe fn as_ptr(&self) -> &UnsafeCell<BinaryNode<T>> {
        &self.root
    }

    /// Gets the root of the tree.
    pub fn root(&self) -> &BinaryNode<T> {
        unsafe {
            (self.root.get() as *const _).as_ref()
                .expect("Binary tree had no root node")
        }
    }

    /// Gets the root of the tree mutably.
    pub fn root_mut(&mut self) -> &mut BinaryNode<T> {
        unsafe {
            (self.root.get()).as_mut().expect("Binary tree had no root node")
        }
    }
}
