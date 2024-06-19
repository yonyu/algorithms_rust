use std::cell::RefCell;
use std::rc::Rc;

pub type NodeLink<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: NodeLink<T>,
}

impl<T: Clone> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
        }))
    }
}