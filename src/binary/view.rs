use std::ptr::null_mut;
use std::ops::Deref;
use std::marker::PhantomData;

use super::binary::{BinaryNode, BinaryTree, Dir};

/// Base view struct. This implements all the main methods used
/// to traverse the binary tree.
pub struct BinaryViewInner<'tree, T: 'tree>{
    node: *mut BinaryNode<T>,
    data: PhantomData<&'tree T>
}

/// Immutable binary tree view.
pub struct BinaryView<'tree, T: 'tree>(BinaryViewInner<'tree, T>);
/// Mutable binary tree view.
/// There should be only one of these at any given time.
pub struct BinaryViewMut<'tree, T:'tree>(BinaryViewInner<'tree, T>);

impl <'tree, T: 'tree> BinaryView<'tree, T> {
    pub fn new(tree: &'tree BinaryTree<T>) -> Self {
        BinaryView(BinaryViewInner {
            node: unsafe { tree.as_ptr().get() },
            data: PhantomData::default()
        })
    }

    /// Mutably borrows the tree, converting the view into a mutable view.
    /// The position within the tree is preserved.
    ///
    /// # Safety
    /// Does not guarantee that this view points to a node in the tree.
    /// IF this view does point to a node in the tree, then this function is safe.
    #[allow(unused_variables)]
    pub unsafe fn into_mut_unchecked(self, tree: &'tree mut BinaryTree<T>) -> BinaryViewMut<'tree, T> {
        BinaryViewMut(self.0)
    }

    /// Mutably borrows the tree, converting the view into a mutable view.
    /// The position within the tree is preserved.
    ///
    /// Time complexity: O(k), where k = height of the tree.
    ///
    /// It must check that the given tree is in fact what is being searched.
    /// If this view is not searching the tree, `None` is returned
    pub fn into_mut(mut self, tree: &'tree mut BinaryTree<T>) -> Result<BinaryViewMut<'tree, T>, BinaryView<'tree, T>> {
        let old_node = self.0.node;
        let root_node = tree.root();
        unsafe {
            loop {
                if self.0.node as *const _ == root_node as *const _ {
                    self.0.node = old_node;
                    return Ok(BinaryViewMut(self.0))
                }
                self = match self.climb() {
                    Ok(this) => this,
                    Err(mut this) => {
                        this.0.node = old_node;
                        return Err(this)
                    }
                }
            }
        }
    }

    /// Attempts to climb up the tree.
    ///
    /// If view is at the root (and thus had no parent),
    /// an `Err` with the view in its original place is returned.
    pub fn climb(mut self) -> Result<Self, Self> {
        if self.0.node == null_mut() {
            panic!("View pointed to an invalid tree");
        }
        unsafe {
            let node = &*self.0.node;
            if node.parent() == null_mut() {
                Err(self)
            } else {
                self.0.node = node.parent();
                Ok(self)
            }
        }
    }


    /// Attempts to descend down the tree in some direction.
    ///
    /// If the node the view is focused on did not have a child in that
    /// direction, an `Err` with the view in its original place is returned.
    pub fn descend(mut self, dir: Dir) -> Result<Self, Self> {
        if self.0.node == null_mut() {
            panic!("View pointed to an invalid tree");
        }
        unsafe {
            let node = &*self.0.node;
            let children = node.children();
            match (dir, children) {
                (Dir::Left, &(Some(ref child), _)) |
                (Dir::Right, &(_, Some(ref child))) => {
                    self.0.node = &*child as *const _ as *mut _;
                    Ok(self)
                },
                _ => Err(self)
            }
        }
    }
}

impl <'tree, T: 'tree> BinaryViewMut<'tree, T> {
    pub fn new(tree: &'tree mut BinaryTree<T>) -> Self {
        BinaryViewMut(BinaryViewInner {
            node: unsafe { tree.as_ptr().get() },
            data: PhantomData::default()
        })
    }
}

impl <'tree, T: 'tree> Deref for BinaryView<'tree, T> {
    type Target = BinaryViewInner<'tree, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <'tree, T: 'tree> Deref for BinaryViewMut<'tree, T> {
    type Target = BinaryViewInner<'tree, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
