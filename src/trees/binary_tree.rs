// An ordered collection of `T`s. isbn: 978-1-492-05259-3, p250
// Binary tree with Box<TreeNode<T>> as the underline data structure.
enum BinaryTree<T: Ord> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T: Ord> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }

    // The `iter` method creates an iterator over the elements of the tree.
    // Isbn: 978-1-492-05259-3, p385
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

// The state of an in-order traversal of a `BinaryTree`.
struct TreeIter<'a, T: Ord> {
    // A stack of references to tree nodes. Since we use `Vec`'s
    // `push` and `pop` methods, the top of the stack is the end of the
    // vector.
    //
    // The node the iterator will visit next is at the top of the stack,
    // with those ancestors still unvisited below it. If the stack is empty,
    // the iteration is over.
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T: 'a + Ord> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T: 'a + Ord> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Ord> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        // Find the node this iteration must produce,
        // or finish the iteration. (Use the `?` operator
        // to return immediately if it's `None`.)
        let node = self.unvisited.pop()?;
        // After `node`, the next thing we produce must be the leftmost
        // child in `node`'s right subtree, so push the path from here
        // down. Our helper method turns out to be just what we need.
        self.push_left_edge(&node.right);
        // Produce a reference to this node's value.
        Some(&node.element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        // Build a small tree.
        let mut tree = BinaryTree::Empty;
        tree.add("jaeger");
        tree.add("robot");
        tree.add("droid");
        tree.add("mecha");
        // Iterate over it.
        let mut v = Vec::new();
        for kind in &tree {
            v.push(*kind);
        }
        assert_eq!(v, ["droid", "jaeger", "mecha", "robot"]);

        assert_eq!(
            tree.iter()
                .map(|name| format!("mega-{}", name))
                .collect::<Vec<_>>(),
            vec!["mega-droid", "mega-jaeger", "mega-mecha", "mega-robot"]
        );
    }
}