/*
Singly linked list implementation in Rust

In this module, we will implement a singly linked list in Rust. A singly linked list is a data
structure that consists of a sequence of nodes, where each node contains a value and a reference
to the next node in the sequence. The first node in the sequence is called the head, and the last
node is called the tail. The tail node's next reference is None, indicating the end of the list.

Basic operations:
new: constructor, creates a new linked list instance
push_front: adds a value to the front of the linked list
push_back: adds a value to the end of the linked list
pop_front: removes and returns the value of the first node in the linked list
pop_back: removes and returns the value of the last node in the linked list

peek_front: returns the value of the first node in the linked list without removing it
peek_tail: returns the value of the last node in the linked list without removing it

insert_at: adds a value at a specific index in the linked list
remove_at: removes a value at a specific index in the linked list
get: returns the value at a specific index in the linked list

iterator:
find: finds a value in the linked list and returns its index
traverse: traverses the linked list and returns a vector of values

The LinkedList struct has two fields: head and count. The head field is an Option<NodeLink<T>>,
which is an alias for Option<Rc<RefCell<Node<T>>>>. The count field is an i32 that keeps track of
the number of nodes in the linked list.
*/
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use crate::linked_lists::linked_list_node::{Node, NodeLink};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct LinkedList<T> {
    head: NodeLink<T>,
    count: i32,
}

impl <T: Clone> LinkedList<T> { // where T is Clone and Debug:  Clone + Debug
    // The associated new_empty method creates a new linked list instance with the head that is
    // set to None and the count set to 0, that is, an empty linked list.
    #[allow(dead_code)]
    pub fn new_empty() -> Self {
        LinkedList {
            head: None,
            count:0,
        }
    }

    // The associated new method creates a new linked list instance with the head that is set to
    // the given node and the count set to 1.
    #[allow(dead_code)]
    pub fn new(node: Rc<RefCell<Node<T>>>) -> Self {
        LinkedList {
            head: Some(node),
            count: 1,
        }
    }

    // Get the head of the linked list
    #[allow(dead_code)]
    pub fn head(&self) -> &NodeLink<T> {
        &self.head
    }

    // Get the count of Nodes in the linked list
    #[allow(dead_code)]
    pub fn count(&self) -> i32 {
        self.count
    }

    #[allow(dead_code)]
    pub fn experiment(&self) {
        let mut list =  std::collections::linked_list::LinkedList::new();
        list.push_front(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
    }

    // adds a value to the front of the linked list
    #[allow(dead_code)]
    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        new_node.borrow_mut().next = self.head.take();

        self.head = Some(new_node);
        self.count += 1;
    }

    // views the first node in the linked list without removing it
    #[allow(dead_code)]
    pub fn peek_head(&mut self) -> Option<Node<T>> {
        self.head.take().map(|node| Rc::try_unwrap(node).ok().unwrap().into_inner())
    }

    // remove and return the value of the first node in the linked list
    #[allow(dead_code)]
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next); // sets the new head to the next node
            } else {
                self.head = None; // there is a single node, the list is empty after popping the head off
            }

            self.count -= 1;

            Rc::try_unwrap(head).ok().unwrap().into_inner().value
        })
    }

    // to traverse the linked list starting from the head.
    // It clones the head to start the traversal and iterates through the list. The while let loop
    // dereferences and borrows the current node, prints its value, and then moves to the next node
    // by cloning its link. This avoids mutable borrowing issues and ensures safe traversal.
    #[allow(dead_code)]
    pub fn traverse(&self) -> Vec<T>{
        let mut current_link = self.head.clone();
        let mut values = Vec::new();
        while let Some(current_node) = current_link {
            let node = current_node.borrow();
            values.push(node.value.clone());
            //println!("{:?}", current_node);
            //println!("value: {:?}", node.clone().value);
            current_link = node.next.clone();
        }

        values
    }

    #[allow(dead_code)]
    pub fn push_back(&mut self, value: T) { // O(n)
        self.push(value);
    }

    // add a value to the end of the linked list
    #[allow(dead_code)]
    pub fn push(&mut self, value: T) { // O(n)
        // traverse the list until the end is reached and then add the new node to the end

        let new_node = Node::new(value);
        self.count += 1;

        match self.head {
            Some(ref head) => {
                let mut current = head.clone();
                loop {
                    let next = {
                        let current_borrowed = current.borrow();
                        current_borrowed.next.clone()
                    };
                    match next {
                        Some(next_node) => {
                            current = next_node;
                        },
                        None => {
                            break;
                        }
                    }
                }
                current.borrow_mut().next = Some(new_node);
            },
            None => {
                self.head = Some(new_node);
            }
        }
    }

    // return the value of the last node in the linked list without removing it
    #[allow(dead_code)]
    pub fn peek_tail(&mut self) -> Option<T> {
        // traverse the list until the end is reached
        let mut current_link = self.head.clone();
        while let Some(current_node) = current_link {
            let node = current_node.borrow();
            match node.clone().next {
                None => return Some(node.value.clone()),
                Some(x) => current_link =  Some(x), //&mut node.next,
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_tail()
    }

    // remove and return the value of the last node in the linked list
    #[allow(dead_code)]
    pub fn pop_tail(&mut self) -> Option<T> {
        let mut current_link = self.head.clone();
        let mut previous_link: NodeLink<T> = None;
        while let Some(current_node) = current_link {
            let node = current_node.borrow();
            if node.clone().next.is_none() { // we have reached the end of the list
                previous_link.take().map(|n| n.borrow_mut().next = None); // sets the next node of the previous link to None
                self.count -= 1;

                //return Some(Rc::try_unwrap(node.borrow()).ok().unwrap().into_inner().value);
                return Some(node.value.clone());
            } else {
                // we are not at the end yet
                previous_link = Some(current_node.clone()); // sets the previous link's next node to None
                current_link = node.next.clone(); // sets the current link to the next node
            }
        }

        None
    }
}

#[allow(dead_code)]
pub struct ListIterator<T> {
    current: NodeLink<T>,
}

impl<T: Clone> ListIterator<T>{
    #[allow(dead_code)]
    fn new(start_at: NodeLink<T>) -> ListIterator<T> {
        ListIterator {
            current: start_at,
        }
    }
}
impl <T: Clone> Iterator for ListIterator<T> {
    type Item = T;

    #[allow(dead_code)]
    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },

            None => None,
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front1() {
        let mut list = LinkedList::new_empty();
        list.push_front(1);
        println!("LIST: {:?}", list);
        println!("HEAD: {:?}", list.head);
        println!("COUNT: {:?}", list.count);
    }

    #[test]
    fn test_push_front2() {
        let mut list = LinkedList::new_empty();
        list.push_front(1);
        list.push_front(2);
        println!("LIST: {:?}", list);
        println!("HEAD: {:?}", list.head);
        println!("COUNT: {:?}", list.count);
    }

    #[test]
    fn test_peek_head() {
        let mut list = LinkedList::new_empty();
        list.push_front(1);
        list.push_front(2);

        let value = list.peek_head();
        println!("PEEK: {:?}", value);
    }

    #[test]
    fn test_peek_tail() {
        let mut list = LinkedList::new_empty();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let value = list.peek_tail();
        println!("PEEK: {:?}", value);
    }
    #[test]
    fn test_traverse() {
        let mut list = LinkedList::new_empty();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let values = list.traverse();

        //println!("traversal: {:?}", values);
        assert_eq!(values, vec![3, 2, 1]);
    }

    #[test]
    fn test_pop_tail() {
        let mut list = LinkedList::new_empty();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let value = list.pop_tail();
        //println!("Popped: {:?}", value);
        assert_eq!(value, Some(1));
        let value = list.pop_tail();
        //println!("Popped: {:?}", value);
        assert_eq!(value, Some(2));
        let value = list.pop_tail();
        //println!("Popped: {:?}", value);
        assert_eq!(value, Some(3));
        //println!("Count: {}", list.count);
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new_empty();
        list.push(1);
        list.push(2);
        list.push(3);
        //println!("{:?}", list);

        let value = list.pop_tail();
        //println!("Popped: {:?}", value);
        assert_eq!(value, Some(3));
        let value = list.pop_tail();
        //println!("Popped: {:?}", value);
        assert_eq!(value, Some(2));
        let value = list.pop_tail();
        //println!("Popped: {:?}", value);
        assert_eq!(value, Some(1));
        //println!("Count: {}", list.count);
        assert_eq!(list.count(), 0);
    }
}
