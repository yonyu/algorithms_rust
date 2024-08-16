use std::{cell::RefCell, rc::Rc};

type TreeNodeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[derive(Debug, PartialEq)]
struct TreeNode<T> {
    value: T,
    // left: TreeNodeLink<T>,
    // right: TreeNodeLink<T>,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[derive(Debug, PartialEq)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(TreeNodeLink<T>)
}

impl<T: Ord + Clone> BinaryTree<T> {
    pub fn new(node: TreeNodeLink<T>) -> Self {
        match node {
            None => BinaryTree::Empty,
            Some(link) => BinaryTree::NonEmpty(Some(link))
        }
    }

    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => *self = BinaryTree::NonEmpty(Some(
                Rc::new(RefCell::new(
                    TreeNode {
                        value: value.clone(),
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty,
                    }
                ))
            )),
            BinaryTree::NonEmpty(ref mut link) => {
                if let Some(rr_node) = link {
                    let mut node = rr_node.borrow_mut();
                    if value <= node.value {
                        node.left.add(value);
                    } else {
                        node.right.add(value);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {

        let tree: BinaryTree<i32> = BinaryTree::new(None);
        assert_eq!(tree, BinaryTree::Empty);

    }

    #[test]
    fn test_add() {
        let mut tree: BinaryTree<i32> = BinaryTree::new(None);
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