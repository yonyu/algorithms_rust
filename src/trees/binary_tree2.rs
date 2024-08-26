// Ordered binary tree implementation with Option<Rc<RefCell<TreeNode<T>>>> as
// the underline data structure.
use std::{cell::RefCell, rc::Rc};

// Define the binary tree data structure.
#[derive(Clone, Debug, PartialEq)]
enum BinaryTree<T>
where
    T: Clone + Ord,
{
    Empty,
    NonEmpty(TreeNodeLink<T>),
}

// Define the tree node data structure.
#[derive(Debug, PartialEq)]
struct TreeNode<T>
where
    T: Clone + Ord,
{
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

// Define the alias for Option<Rc<RefCell<TreeNode<T>>>>.
type TreeNodeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

// Define the tree iterator data structure.
pub struct TreeIter<T>
where
    T: Clone + Ord,
{
    unvisited: Vec<Rc<RefCell<TreeNode<T>>>>,
}

// implement the binary tree data structure.
impl<T> BinaryTree<T>
where
    T: Clone + Ord,
{
    fn new() -> Self {
        BinaryTree::Empty
    }

    // This method requires T to implement the Ord traits
    fn add(&mut self, value: T) {
        match self {
            BinaryTree::Empty => {
                let new_node = TreeNode {
                    value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                };
                *self = BinaryTree::NonEmpty(Some(Rc::new(RefCell::new(new_node))));
            }
            BinaryTree::NonEmpty(ref mut link) => {
                if let Some(node_rc) = link {
                    let mut node = node_rc.borrow_mut();
                    if value < node.value {
                        node.left.add(value);
                    } else {
                        node.right.add(value);
                    }
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Clone + Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }
    }
}

// Implement the tree iterator data structure.
// This is done by using in-order traversal to iterate the binary tree
// by pushing the leftmost nodes to the stack.
// and then pop the nodes from the stack and push the right child
// to the stack.
impl<T> TreeIter<T>
where
    T: Clone + Ord,
{
    fn new(tree: &BinaryTree<T>) -> Self {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_leftmost(tree);
        iter
    }

    // The method requires the BinaryTree to implement the Clone trait,
    // and the TreeNode to implement the Clone trait.
    // It also requires T to implement the Clone trait.
    fn push_leftmost(&mut self, tree: &BinaryTree<T>) {
        let mut current = tree.clone();
        while let BinaryTree::NonEmpty(ref link) = current {
            if let Some(node_rc) = link {
                self.unvisited.push(Rc::clone(node_rc));

                let left_child = node_rc.borrow().left.clone();
                current = left_child;
            }
        }
    }
}

// implement the iterator trait for the tree iterator.
impl<T> Iterator for TreeIter<T>
where
    T: Clone + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node_rc) = self.unvisited.pop() {
            let node = node_rc.borrow();
            self.push_leftmost(&node.right);
            Some(node.value.clone())
        } else {
            None
        }
    }
}

// implement the IntoIterator trait for the binary tree.
impl<T> IntoIterator for BinaryTree<T>
where
    T: Clone + Ord,
{
    type Item = T;
    type IntoIter = TreeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter::new(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree, BinaryTree::Empty);
    }

    #[test]
    fn test_add() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.add(5);
        tree.add(3);
        tree.add(7);

        let _expected = BinaryTree::NonEmpty(Some(Rc::new(RefCell::new(TreeNode {
            value: 5,
            left: BinaryTree::NonEmpty(Some(Rc::new(RefCell::new(TreeNode {
                value: 3,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })))),
            right: BinaryTree::NonEmpty(Some(Rc::new(RefCell::new(TreeNode {
                value: 7,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })))),
        }))));
    }
}
