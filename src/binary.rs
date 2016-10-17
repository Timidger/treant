use std::ptr::null_mut;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::marker::PhantomData;

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

/// Base view struct. This implements all the main methods used
/// to traverse the binary tree.
struct BinaryView<'a, T: 'a>{
    node: *mut BinaryNode<T>,
    data: PhantomData<&'a T>
}

/// Immutable binary tree view.
struct BinaryViewRef<'a, T: 'a>(BinaryView<'a, T>);
/// Mutable binary tree view.
/// There should be only one of these at any given time.
struct BinaryViewRefMut<'a, T:'a>(BinaryView<'a, T>);

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

impl <'a, T: 'a> BinaryViewRef<'a, T> {
    fn to_raw(self) -> BinaryView<'a, T> {
        self.0
    }
}

impl <'a, T: 'a> Deref for BinaryViewRef<'a, T> {
    type Target = BinaryView<'a, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <'a, T: 'a> Deref for BinaryViewRefMut<'a, T> {
    type Target = BinaryView<'a, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
    pub fn view(&self) -> BinaryViewRef<T> {
        BinaryViewRef(BinaryView{
            node: self.root.get() as *mut _,
            data: PhantomData::default()
        })
    }

    /// Constructs a mutable view into the binary tree.
    /// You can only have one of these at a time, and you can not have
    /// any immutable views active alongside them mutable view.
    pub fn view_mut(&mut self) -> BinaryViewRefMut<T> {
        BinaryViewRefMut(BinaryView{
            node: self.root.get() as *mut _,
            data: PhantomData::default()
        })
    }

    /// Constructs a mutable view into the binary tree, using the remenants of a
    /// previous view.
    ///
    /// This allows you to efficiently immutabley traverse a tree and then grabbing a
    /// (statically verified) unique mutable view.
    pub fn view_mut_from_raw<'a>(&'a mut self, view: BinaryView<'a, T>) -> BinaryViewRefMut<T> {
        BinaryViewRefMut(view)
    }
}
