use super::binary::{BinaryNode, BinaryTree};
use std::ops::Deref;
use std::marker::PhantomData;

/// Base view struct. This implements all the main methods used
/// to traverse the binary tree.
pub struct BinaryViewInner<'a, T: 'a>{
    node: *mut BinaryNode<T>,
    data: PhantomData<&'a T>
}

/// Immutable binary tree view.
pub struct BinaryView<'a, T: 'a>(BinaryViewInner<'a, T>);
/// Mutable binary tree view.
/// There should be only one of these at any given time.
pub struct BinaryViewMut<'a, T:'a>(BinaryViewInner<'a, T>);

impl <'a, T: 'a> BinaryView<'a, T> {
    pub fn new(tree: &'a BinaryTree<T>) -> Self {
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
    pub unsafe fn into_mut_unchecked(self, tree: &'a mut BinaryTree<T>) -> BinaryViewMut<'a, T> {
        BinaryViewMut(self.0)
    }

    /// Mutably borrows the tree, converting the view into a mutable view.
    /// The position within the tree is preserved.
    ///
    /// Time complexity: O(k), where k = height of the tree.
    ///
    /// It must check that the given tree is in fact what is being searched.
    /// If this view is not searching the tree, `None` is returned
    pub fn into_mut(self, tree: &'a mut BinaryTree<T>) -> Option<BinaryViewMut<'a, T>> {
        let root_node = tree.root();
        let cur_node = self.0.node;
        unsafe {
            while let Some(cur_node) = cur_node.as_ref() {
                if cur_node as *const _ == root_node as *const _ {
                    return Some(BinaryViewMut(self.0))
                }
            }
            None
        }
    }
}

impl <'a, T: 'a> BinaryViewMut<'a, T> {
    pub fn new(tree: &'a mut BinaryTree<T>) -> Self {
        BinaryViewMut(BinaryViewInner {
            node: unsafe { tree.as_ptr().get() },
            data: PhantomData::default()
        })
    }
}

impl <'a, T: 'a> Deref for BinaryView<'a, T> {
    type Target = BinaryViewInner<'a, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <'a, T: 'a> Deref for BinaryViewMut<'a, T> {
    type Target = BinaryViewInner<'a, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
