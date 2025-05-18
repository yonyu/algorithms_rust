/*

// Ordered binary tree implementation with Option<Rc<RefCell<TreeNode<T>>>> as
// the underline data structure.
use std::{cell::RefCell, rc::Rc};

// Define the binary tree data structure.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryTree<T>
where
    T: Clone + Ord,
{
    count: usize,
    head: TreeNodeLink<T>,
}

// Define the tree node data structure.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode<T>
where
    T: Clone + Ord,
{
    value: T,
    left: TreeNodeLink<T>,
    right: TreeNodeLink<T>,
}

// Define the alias for Option<Rc<RefCell<TreeNode<T>>>>.
pub type TreeNodeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

// Define the tree iterator data structure.
pub struct TreeIter<T>
where
    T: Clone + Ord,
{
    pub unvisited: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> BinaryTree<T>
where
    T: Clone + Ord,
{
    pub fn new() -> Self {
        BinaryTree {
            count: 0,
            head: None,
        }
    }

    pub fn add(&mut self, value: T) {
        match self.head {
            None => {
                let new_node = Rc::new(RefCell::new(TreeNode {
                    value,
                    left: None,
                    right: None,
                }));
                self.head = Some(new_node);
            }
            Some(ref mut node) => {
                if value < node.borrow().value {
                    if let Some(left) = &mut node.borrow().left {
                        left.borrow_mut().add(value);
                    } else {
                        let new_node = TreeNode {
                            value,
                            left: None,
                            right: None,
                        };
                        node.borrow_mut().left = Some(Rc::new(RefCell::new(new_node)));
                    }
                } else {
                    if let Some(right) = &mut node.borrow().right {
                        right.borrow_mut().add(value);
                    } else {
                        let new_node = TreeNode {
                            value,
                            left: None,
                            right: None,
                        };
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(new_node)));
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_leftmost(self);
        iter
    }
}

impl<T> TreeNode<T>
where
    T: Clone + Ord,
{
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, value: T) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.borrow_mut().add(value);
            } else {
                let new_node = TreeNode {
                    value,
                    left: None,
                    right: None,
                };
                self.left = Some(Rc::new(RefCell::new(new_node)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.borrow_mut().add(value);
            } else {
                let new_node = TreeNode {
                    value,
                    left: None,
                    right: None,
                };
                self.right = Some(Rc::new(RefCell::new(new_node)));
            }
        }
    }
}

impl<T> TreeIter<T>
where
    T: Clone + Ord,
{
    pub fn new(tree: &BinaryTree<T>) -> Self {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_leftmost(tree);
        iter
    }

    fn push_leftmost(&mut self, tree: &BinaryTree<T>) {
        let mut current = tree;
        while let BinaryTree::NonEmpty(ref link) = *current {
            self.unvisited.push(Rc::new(RefCell::new(link.clone())));
            current = &link;
        }
    }
}

impl<T> Iterator for crate::trees::binary_tree3::TreeIter<T>
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
*/

/*

// Ordered binary tree implementation with Option<Rc<RefCell<TreeNode<T>>>> as
// the underline data structure.
use std::{cell::RefCell, rc::Rc};

// Define the binary tree data structure.
#[derive(Clone, Debug, PartialEq)]
enum BinaryTree<T> where T: Clone {
    Empty,
    NonEmpty(TreeNodeLink<T>),
}

#[derive(Debug, PartialEq)]
struct TreeNode<T> where T: Clone {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

type TreeNodeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

pub struct TreeIter<T> where T: Clone {
    unvisited: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeIter<T>  where T: Clone {
    fn new(tree: &BinaryTree<T>) -> Self {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_leftmost(tree);
        iter
    }

    fn push_leftmost(&mut self, tree: &BinaryTree<T>) {
        let mut current = tree;
        while let BinaryTree::NonEmpty(ref link) = *current {
            if let Some(node_rc) = link {
                self.unvisited.push(Rc::clone(node_rc));

                // pass the reference to the left child to `current`
                //current = &node_rc.borrow().left;

                let left_child = node_rc.borrow().left.clone();
                current = &left_child;

                //current = &node_rc.borrow().left;
                //let node_ref = node_rc.borrow();
                //current = &node_ref.clone().left;


                // Move to the left child without borrowing `node_rc`
                // let left_child = node_rc.borrow().left.clone();
                // current = &left_child;

            }
        }
    }
}

impl<T> Iterator for TreeIter<T>  where T: Clone {
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

impl<T> IntoIterator for BinaryTree<T> where T: Clone {
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

        let _expected = BinaryTree::NonEmpty(Some(
            Rc::new(RefCell::new(
                TreeNode {
                    value: 5,
                    left: BinaryTree::NonEmpty(Some(
                        Rc::new(RefCell::new(
                            TreeNode {
                                value: 3,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }
                        ))
                    )),
                    right: BinaryTree::NonEmpty(Some(
                        Rc::new(RefCell::new(
                            TreeNode {
                                value: 7,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }
                        ))
                    )),
                }
            ))
        ));
    }
}
















// Ordered binary tree implementation with Option<Rc<RefCell<TreeNode<T>>>> as
// the underline data structure.
use std::{cell::RefCell, rc::Rc};

// Define the binary tree data structure.
#[derive(Clone, Debug, PartialEq)]
enum BinaryTree<T> where T: Clone {
    Empty,
    NonEmpty(TreeNodeLink<T>),
}

#[derive(Debug, PartialEq)]
struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

type TreeNodeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

pub struct TreeIter<T> {
    unvisited: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeIter<T> {
    fn new(tree: &BinaryTree<T>) -> Self {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_leftmost(tree);
        iter
    }

    fn push_leftmost(&mut self, tree: &BinaryTree<T>) {
        let mut current = tree;
        while let BinaryTree::NonEmpty(ref link) = *current {
            if let Some(node_rc) = link {
                self.unvisited.push(Rc::clone(node_rc));
                //current = &node_rc.borrow().left;
                //let node_ref = node_rc.borrow();
                //current = &node_ref.clone().left;


                // Move to the left child without borrowing `node_rc`
                let left_child = node_rc.borrow().left.clone();
                current = &left_child;

            }
        }
    }
}

impl<T: Clone> Iterator for TreeIter<T> {
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

impl<T: Clone> IntoIterator for BinaryTree<T> {
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

        let _expected = BinaryTree::NonEmpty(Some(
            Rc::new(RefCell::new(
                TreeNode {
                    value: 5,
                    left: BinaryTree::NonEmpty(Some(
                        Rc::new(RefCell::new(
                            TreeNode {
                                value: 3,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }
                        ))
                    )),
                    right: BinaryTree::NonEmpty(Some(
                        Rc::new(RefCell::new(
                            TreeNode {
                                value: 7,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }
                        ))
                    )),
                }
            ))
        ));
    }
}


 */










/*

// Ordered binary tree implementation with Option<Rc<RefCell<TreeNode<T>>>> as
// the underline data structure.
use std::{cell::RefCell, rc::Rc};
use std::iter::{IntoIterator, Iterator};

// Define the binary tree data structure.

#[derive(Debug, PartialEq)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(TreeNodeLink<T>)
}

#[derive(Debug, PartialEq)]
struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

type TreeNodeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

impl <T: Ord + Clone> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }
    }

    // Create a new node link.
    // This is a helper function that creates a new node link.
    fn new_node_link (value: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            value,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        })))
    }
}

impl<T: Ord + Clone> BinaryTree<T> {
    // Create a new empty binary tree.
    pub fn new() -> Self {
        BinaryTree::Empty
    }

    // Add a new value to the tree.
    pub fn add(&mut self, value: T) {
        match self {
            BinaryTree::Empty => *self = Self::NonEmpty(TreeNode::new_node_link(value)),
            BinaryTree::NonEmpty(ref mut link) => {
                let node = link.as_ref().unwrap();
                let mut node_ref = node.borrow_mut();
                if value <= node_ref.value {
                    node_ref.left.add(value);
                } else {
                    node_ref.right.add(value);
                }
            }
        }
    }

    fn iter(self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(&self);
        iter
    }
}

// Implement iterator for the binary tree.

pub struct TreeIter<T> {
    unvisited: Vec<TreeNode<T>>,
}

impl<T> TreeIter<T> {
    // Push nodes in the left edge of the tree to the unvisited stack.
    fn push_left_edge(&mut self, mut tree: &BinaryTree<T>) {
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node.clone().into().as_ref());
            tree = node.as_ref().unwrap();
        }
    }
}

impl<T> Iterator for TreeIter<T> {
    type Item = TreeNode<T>;
    //type Item = Option<TreeNode<T>>;

    // Get the next node in the tree and push the left edge of the right child to the unvisited stack.
    // Return the next node in the tree.
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop();
        if let Some(x) = &node {
            self.push_left_edge(&x.right);
        }
        node
    }
}

impl<T> IntoIterator for BinaryTree<T> {
    type Item = TreeNode<T>;
    type IntoIter = TreeIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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

        let _expected = BinaryTree::NonEmpty(Some(
            Rc::new(RefCell::new(
                TreeNode {
                    value: 5,
                    left: BinaryTree::NonEmpty(Some(
                        Rc::new(RefCell::new(
                            TreeNode {
                                value: 3,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }
                        ))
                    )),
                    right: BinaryTree::NonEmpty(Some(
                        Rc::new(RefCell::new(
                            TreeNode {
                                value: 7,
                                left: BinaryTree::Empty,
                                right: BinaryTree::Empty,
                            }
                        ))
                    )),
                }
            ))
        ));
    }
}

 */