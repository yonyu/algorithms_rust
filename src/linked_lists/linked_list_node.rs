use std::cell::RefCell;
use std::rc::Rc;

// The NodeLink type is an alias for Option<Rc<RefCell<Node<T>>>>, which is used to
// represent the next node in the list.
pub type NodeLink<T> = Option<Rc<RefCell<Node<T>>>>;

// The Node struct has two fields: value, which stores the data of the node, and next,
// which stores the reference to the next node in the list. The value field is of type T,
// which is a generic type parameter. The next field is of type NodeLink<T>, which is an
// alias for Option<Rc<RefCell<Node<T>>>>. This allows the next field to be either Some or None.
//
// Storing each node item in a Rc<RefCell<T>> provides the ability to retrieve and replace
// data as needed (the internal mutability pattern)—crucial when executing operations on the
// list.

#[derive(Clone, Debug)] // The Node struct also derives the Clone and Debug traits.
pub struct Node<T> {
    pub value: T,
    pub next: NodeLink<T>,
}

// impl PartialEq for Node<i32> {
//     fn eq(&self, other: &Self) -> bool {
//         self.value == other.value
//     }
// }

// impl<T: Clone + Eq> PartialEq for Node<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.value == other.value && self.next == other.next
//     }
// }

// The Node struct has three methods: new, value and next.
impl<T: Clone> Node<T> {
    // The associated new method creates a new node with the given value and returns a
    // reference-counted smart pointer to a RefCell that wraps the node.
    // #[allow(dead_code)]
    // pub fn new(value: T) -> Rc<RefCell<Node<T>>> {
    //     Rc::new(RefCell::new(Node {
    //         value,
    //         next: None,
    //     }))
    // }

    #[allow(dead_code)]
    pub fn new(value: T, next: NodeLink<T>) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next,
        }))
    }

    // The value method returns a reference to the value of the node.
    #[allow(dead_code)]
    pub fn value(&self) -> &T {
        &self.value
    }

    // The next method returns a reference to the next node in the list.
    #[allow(dead_code)]
    pub fn next(&self) -> &NodeLink<T> {
        &self.next
    }
}