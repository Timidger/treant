use std::ptr::null_mut;
use std::cell::UnsafeCell;
use std::mem;

pub type Child<T> = Option<Box<BinaryNode<T>>>;
pub type Children<T> = (Child<T>, Child<T>);

/// The direction in the tree to go.
/// Left refers to the first element.
/// Right refers to the second element.
pub enum Dir {
    Left,
    Right
}

/// A tree where each node has 0, 1, or 2 children.
#[derive(Debug)]
pub struct BinaryTree<T> {
    root: UnsafeCell<BinaryNode<T>>
}

/// A node that has 0, 1, or 2 children and maybe a parent.
/// If the node has no parent, it is the root of the tree.
#[derive(Debug, Eq, PartialEq)]
pub struct BinaryNode<T> {
    parent: *mut BinaryNode<T>,
    children: Children<T>,
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

    /// Gets the value behind the node.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Sets the value of the node to the given data.
    /// Returns the data that was there previously.
    pub fn set_value(&mut self, data: T) -> T {
        mem::replace(&mut self.value, data)
    }

    /// Gets a reference to the children of this node in the tree.
    pub fn children(&self) -> &Children<T> {
        &self.children
    }

    /// Replaces the left/right child (if any) of this node with the given value.
    /// The previous child (if any) is returned.
    pub fn add_child(&mut self, dir: Dir, data: T) -> Child<T> {
        let new_node = Box::new(BinaryNode::new(data));
        let child = match dir {
            Dir::Left  => &mut self.children.0,
            Dir::Right => &mut self.children.1
        };
        mem::replace(child, Some(new_node))
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


#[cfg(tests)]
mod tests {
    use super::{BinaryTree, BinaryNode, Dir};
    use std::ptr::null_mut;

    /// Outputs a binary tree with only one element at the root:
    /// a node with `0` as its value.
    fn empty_binary() -> BinaryTree<i32> {
        BinaryTree::new(0)
    }

    #[test]
    fn adding_to_tree() {
        let mut empty_t = empty_binary();
        let root = empty_t.root_mut();
        let null_parent = null_mut();
        assert_eq!(root.children(), &(None, None));

        root.add_child(Dir::Left, 1);
        assert_eq!(root.children(),
                   &(Some(Box::new(BinaryNode {
                       parent: null_parent,
                       children: (None, None),
                       value: 1
                   })),
                    None));
        root.add_child(Dir::Left, 2);
        assert_eq!(root.children(),
                   &(Some(Box::new(BinaryNode{
                       parent: null_parent,
                       children: (None, None),
                       value: 2
                   })),
                    None));
        root.add_child(Dir::Right, 3);
        assert_eq!(root.children(),
                   &(Some(Box::new(BinaryNode{
                       parent: null_parent,
                       children: (None, None),
                       value: 2
                   })),
                    Some(Box::new(BinaryNode{
                        parent: null_parent,
                        children: (None, None),
                        value: 3
                    }))));
    }

    #[test]
    fn unsafe_cell_points_to_root() {
        let mut empty_t = empty_binary();
        let tree_ptr = unsafe { empty_t.as_ptr() }.get();
        {
            let root_mut = empty_t.root_mut();
            assert_eq!(tree_ptr, root_mut as *mut _);
        }
        {
            let root = empty_t.root();
            assert_eq!(tree_ptr as *const _, root as *const _);
        }
    }

    #[test]
    fn value_is_set() {
        let mut empty_t = empty_binary();
        {
            let root = empty_t.root_mut();
            assert_eq!(*root.value(), 0);
            assert_eq!(root.set_value(1), 0);
            assert_eq!(*root.value(), 1);
            assert_eq!(root.set_value(-1), 1);
            assert_eq!(*root.value(), -1);
        }
        let root = empty_t.root();
        assert_eq!(*root.value(), -1);
    }
}
