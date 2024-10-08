/*
Singly linked list implementation in Rust

In this module, we will implement a singly linked list in Rust. A singly linked list is a data
structure that consists of a sequence of nodes, where each node contains a value and a reference
to the next node in the sequence. The first node in the sequence is called the head, and the last
node is called the tail. The tail node's next reference is None, indicating the end of the list.

Basic operations:

new: constructor, creates a new empty linked list instance
from: creates a new linked list instance from a Vec<T>
from_value: creates a new linked list instance with a single node containing the given value
from_node: creates a new linked list instance with the head that is set to the given node

head: returns the head of the linked list
tail: returns the tail of the linked list
count: returns the number of nodes in the linked list

push_front: adds a value to the front of the linked list
pop_front: removes and returns the value of the first node in the linked list

push_back: adds a value to the end of the linked list
pop_back: removes and returns the value of the last node in the linked list

peek_head: returns the first node in the linked list without removing it
peek_tail: returns the last node in the linked list without removing it

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
    tail: NodeLink<T>, // not implemented yet
    count: i32,
}

impl <T: Clone> LinkedList<T> { // where T is Clone and Debug:  Clone + Debug
    // The associated new_empty method creates a new linked list instance with the head that is
    // set to None and the count set to 0, that is, an empty linked list.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    // The associated from_node method creates a new linked list instance with the head that is
    // set to the given node and the count set to 1.
    #[allow(dead_code)]
    pub fn from_node(node: Rc<RefCell<Node<T>>>) -> Self {
        LinkedList {
            head: Some(node.clone()),
            tail: Some(node),
            count: 1,
        }
    }

    // The associated from_node method creates a new linked list instance with the head that is
    // set to the node that is created with the given value and the count set to 1.
    #[allow(dead_code)]
    pub fn from_value(value: T) -> Self {
        let new_node = Rc::new(RefCell::new(Node {value, next: None}));

        LinkedList {
            head: Some(new_node.clone()),
            tail: Some(new_node),
            count: 1,
        }
    }

    // The associated from method creates a new linked list instance from a give Vec<T>
    #[allow(dead_code)]
    pub fn from(values: &Vec<T>) -> Self {
        let mut instance = LinkedList::new();

        for value in values.iter().rev() {
            instance.push_front(value.clone());
        }

        instance
    }

    // Get the head of the linked list
    #[allow(dead_code)]
    pub fn head(&self) -> NodeLink<T> {
        // self.head is of type NodeLink<T>, which is defined as Option<Rc<RefCell<Node<T>>>>.
        //
        // 1. If self.head is None, self.head.clone() returns None.
        // 2. If self.head is Some(Rc<RefCell<Node<T>>>), self.head.clone() returns a new
        //    Option<Rc<RefCell<Node<T>>>> with a cloned Rc, which points to the same Node<T>
        //    inside the RefCell.
        // 3. The reference count for the Rc is incremented by 1 when you clone it, allowing
        //    shared ownership of the Node.
        // 4. When you clone a Rc, you are not cloning the underlying data. Instead, you are
        //    incrementing the reference count of the Rc and getting a new Rc instance that
        //    points to the same data.
        // 5. The RefCell itself is not cloned when you clone the Rc. Instead, the Rc still
        //    points to the same RefCell.
        self.head.clone()
    }

    // Get the tail of the linked list
    pub fn tail(&self) -> NodeLink<T> {
        self.tail.clone()
    }

    // Get the count of Nodes in the linked list
    #[allow(dead_code)]
    pub fn count(&self) -> i32 {
        self.count
    }

    // adds a value to the front of the linked list
    #[allow(dead_code)]
    pub fn push_front(&mut self, value: T) {
        // let new_node = Node::new(value, None);
        // new_node.borrow_mut().next = self.head.take();
        let new_node = Node::new(value, self.head.take());

        //self.head = Some(new_node.clone());

        // if tail is None
        if self.tail.is_none() {
            self.tail = Some(new_node.clone());
        }

        self.head = Some(new_node);

        self.count += 1;
    }

    // views the first node in the linked list without removing it
    #[allow(dead_code)]
    pub fn peek_head(&mut self) -> Option<Node<T>> {
        //self.head.take().map(|node| Rc::try_unwrap(node).ok().unwrap().into_inner())
        // Check if the head exists
        self.head.as_ref().map(|head_node_rc| {
            // Borrow the node immutably
            let head_node = head_node_rc.borrow();
            // Clone the node's contents and return it as an `Option<Node<T>>`
            Node {
                value: head_node.value.clone(),
                next: None,  // We don't want to clone the whole list, just the head node
            }
        })
    }

    // remove and return the value of the first node in the linked list
    #[allow(dead_code)]
    pub fn pop_front(&mut self) -> Option<T> {
        // Check if the list is empty
        if let Some(head) = self.head.take() {
            // Borrow the inner value of the current head safely
            let head_value = Rc::clone(&head).borrow().value.clone();

            // Move the head to the next node
            self.head = head.borrow_mut().next.take();

            // If the list is now empty after removing the head, also set the tail to None
            if self.head.is_none() {
                self.tail.take();
            }

            self.count -= 1;

            // Return the value of the removed head node
            Some(head_value)
        } else {
            // If the list was empty, return None
            None
        }
    }

    // Should implement iterator.
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

    // add a value to the end of the linked list
    #[allow(dead_code)]
    pub fn push_back(&mut self, value: T) { // O(n)
        //self.push_not_using_tail(value);
        // check if the tail is None
        let new_node = Node::new(value, None);
        self.count += 1;

        // if tail is not None, set the next node of the tail to the new node
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node);

        } else {
            // If the list is empty, set both head and tail to the new node
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }

    // alternative implementation to add a value to the end of the linked list
    // locates the tail node and then adds the new node to the end (not efficient)
    #[allow(dead_code)]
    pub fn push_back_2(&mut self, value: T) { // O(n)
        // traverse the list until the end is reached and then add the new node to the end

        let new_node = Node::new(value, None);
        self.count += 1;
        self.tail = Some(new_node.clone());

        match self.head {
            Some(ref head) => {
                // The list is not empty, traverse to the end and add the new node
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
                // The list is empty, set the head to the new node
                self.head = Some(new_node);
            }
        }
    }

    // view the last node in the linked list without removing it
    #[allow(dead_code)]
    pub fn peek_tail(&mut self) -> Option<Node<T>> {
        // Check if the tail exists
        self.tail.as_ref().map(|tail_node_rc| {
            // Borrow the node immutably
            let tail_node = tail_node_rc.borrow();
            // Clone the node's contents and return it as an Option<Node<T>>
            Node {
                value: tail_node.value.clone(),
                next: None,  // We don't want to clone the whole list, just the tail node
            }
        })
    }

    // view the value of the last node in the linked list without removing it
    // by traversing the list until the end is reached, not efficient.
    #[allow(dead_code)]
    pub fn peek_tail_value(&mut self) -> Option<T> {
        // traverse the list until the end is reached
        let mut current_link = self.head.clone();
        while let Some(current_node) = current_link {
            let node = current_node.borrow();
            match node.clone().next {
                None => return Some(node.value.clone()),
                Some(x) => current_link =  Some(x),
            }
        }
        None
    }

    // removes the last node and returns its value. it does the same thing as pop_tail
    #[allow(dead_code)]
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_tail_2()
    }

    // removes the last node and returns its value
    #[allow(dead_code)]
    pub fn pop_tail(&mut self) -> Option<T> {
        // check if the list is empty
        if self.head.is_none() {
            return None;
        }

        // check if the list has a single node
        //if self.head.as_ref() == self.tail.as_ref() { // requires deriving PartialEq for Node<T>
        if self.head.as_ref().unwrap().borrow().next.is_none() {
            //return self.pop_front();
            let head = self.head.take()?;
            self.tail.take();
            self.count -= 1;

            return Some(head.borrow().value.clone());
        }

        // The list has more than one node, iterate to find the second-to-last node
        let mut current = self.head.as_ref().unwrap().clone();
        while current.borrow().next.as_ref().unwrap().borrow().next.is_some() {
            //current = current.borrow().next.as_ref().unwrap().clone(); // not working
            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next;
        }

        // At this point, `current` is the second-to-last node
        let tail = current.borrow_mut().next.take()?; // current.borrow_mut().next becomes None after calling take()
        //let tail_value = Rc::try_unwrap(tail).ok().unwrap().into_inner().value;// not working

        // Borrow the inner value of the current head safely
        let tail_value = Rc::clone(&tail).borrow().value.clone();

        self.count -= 1;
        // Update `self.tail` to point to the new tail (which is `current`)
        self.tail = Some(current);

        Some(tail_value)
    }

    // the alternative way to pop the tail
    // removes the last node and returns its value
    #[allow(dead_code)]
    pub fn pop_tail_2(&mut self) -> Option<T> {
        let mut current_link = self.head.clone();
        let mut previous_link: NodeLink<T> = None;
        while let Some(current_node) = current_link {
            let node = current_node.borrow();
            if node.clone().next.is_none() { // we have reached the end of the list
                // Updates the list

                self.count -= 1;
                // check if the list has a single node
                if self.count == 0 {
                    // list will be empty after removing the last node
                    self.head = None;
                    self.tail = None;
                } else {
                    // the list has more than one node, update the tail
                    self.tail = previous_link.clone(); // the previous link becomes the last node, updating the tail with its value
                }
                // sets the next node of the previous link to None. note: previous_link becomes None after calling take()
                previous_link.take().map(|n| n.borrow_mut().next = None);

                //return Some(Rc::try_unwrap(node.borrow()).ok().unwrap().into_inner().value);// not working
                return Some(node.value.clone());
            } else {
                // we are not at the end yet
                previous_link = Some(current_node.clone()); // sets the previous link's next node to None
                current_link = node.next.clone(); // sets the current link to the next node
            }
        }

        None
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            current: self.head.clone()
        }
    }
}


#[allow(dead_code)]
pub struct LinkedListIter<T> {
    current: NodeLink<T>,
}

impl<'a, T: 'a + Clone> LinkedListIter<T>{
    #[allow(dead_code)]
    fn new(start_at: NodeLink<T>) -> LinkedListIter<T> {
        LinkedListIter {
            current: start_at,
        }
    }
}

impl <T: Clone> Iterator for LinkedListIter<T> {
    type Item = T;

    #[allow(dead_code)]
    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        // update self.current to the next node
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

impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;

    type IntoIter = LinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = LinkedList::<i32>::new();
        assert_eq!(list.count(), 0);
        match list.head() {
            None => assert!(true),
            _ => assert!(false),
        }

        match list.tail() {
            None => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_from_node() {
        let node = Node::new(1, None);
        let list = LinkedList::from_node(node);
        assert_eq!(list.count(), 1);

        // check the head and tail are the same
        are_equal_head_tail(& list);

        // this requires Node<T> to implement PartialEq
        //assert_eq!(list.tail(), list.head());
    }

    #[test]
    fn test_from_value() {
        let list = LinkedList::from_value(1);
        assert_eq!(list.count(), 1);

        // check the head and tail are the same
        are_equal_head_tail(& list);
    }

    #[test]
    fn test_from() {
        let mut list = LinkedList::from(&vec![1, 2, 3, 4, 5]);
        assert_eq!(list.count(), 5);

        let head = list.peek_head().unwrap();
        assert_eq!(head.value, 1);
        let tail = list.peek_tail().unwrap();
        assert_eq!(tail.value, 5);
    }

    #[test]
    fn test_push_front(){
        let mut list = LinkedList::new();
        list.push_front(1);
        assert_eq!(list.count(), 1);

        // check the head and tail are the same
        are_equal_head_tail(& list);
    }

    fn are_equal_head_tail(list: & LinkedList<i32>) {
        match list.head() {
            Some(head) => {
                let head = head.borrow();
                assert_eq!(*head.value(), 1);

                match list.tail() {
                    Some(tail) => {
                        let tail = tail.borrow();
                        //assert_eq!(*tail.value(), 1);
                        assert_eq!(*head.value(), *tail.value());
                    },
                    None => assert!(false),
                }
            },
            None => assert!(false),
        }
    }

    #[test]
    fn test_push_front_2() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);

        let head = list.peek_head().unwrap();
        assert_eq!(head.value, 2);
        let tail = list.peek_tail().unwrap();
        assert_eq!(tail.value, 1);

        assert_eq!(list.count(), 2);
    }

    #[test]
    fn test_pop_front() { //**************
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let value = list.pop_front();
        assert_eq!(value, Some(3));
        let tail = list.peek_tail().unwrap();
        assert_eq!(tail.value, 1);

        let value = list.pop_front();
        assert_eq!(value, Some(2));
        let tail = list.peek_tail().unwrap();
        assert_eq!(tail.value, 1);

        let value = list.pop_front();
        assert_eq!(value, Some(1));
        assert_eq!(list.count(), 0);

        match list.peek_tail() {
            None => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_peek_head() {
        let mut list = LinkedList::new();
        let node = list.peek_head();
        match node {
            None => assert!(true),
            _ => assert!(false),
        }

        list.push_front(1);
        let node = list.peek_head().unwrap();
        assert_eq!(node.value, 1);

        list.push_front(2);
        let head = list.peek_head().unwrap();
        assert_eq!(head.value, 2);
    }

    #[test]
    fn test_peek_tail() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let value = list.peek_tail_value();
        assert_eq!(value, Some(1));
    }

    #[test]
    fn test_traverse() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let values = list.traverse();
        assert_eq!(values, vec![3, 2, 1]);
    }

    #[test]
    fn test_pop_tail_1() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let value = list.pop_tail();
        assert_eq!(value, Some(1));

        let value = list.pop_tail();
        assert_eq!(value, Some(2));

        let value = list.pop_tail();
        assert_eq!(value, Some(3));
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_pop_tail_2() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let value = list.pop_tail();
        assert_eq!(value, Some(3));

        let value = list.pop_tail();
        assert_eq!(value, Some(2));

        let value = list.pop_tail();
        assert_eq!(value, Some(1));
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_pop_tail2_1() {
        let mut list = LinkedList::new();
        assert!(list.tail.is_none());
        assert!(list.head.is_none());

        list.push_back(1);
        assert!(list.head.is_some());
        assert!(list.tail.is_some());

        list.push_back(2);
        list.push_back(3);
        // check the head and tail are correct
        let head = list.peek_head().unwrap();
        assert_eq!(head.value, 1);
        let tail = list.peek_tail().unwrap();
        assert_eq!(tail.value, 3);

        let value = list.pop_tail_2();
        assert_eq!(value, Some(3));

        // check the head and tail are correct
        let head = list.peek_head().unwrap();
        assert_eq!(head.value, 1);
        let tail = list.peek_tail().unwrap();
        assert_eq!(tail.value, 2);

        let value = list.pop_tail_2();
        //println!("Popped: {:?}", value);
        assert_eq!(value, Some(2));

        // check the head and tail are correct
        let head = list.peek_head().unwrap();
        assert_eq!(head.value, 1);
        let tail = list.peek_tail().unwrap();
        assert_eq!(tail.value, 1);


        let value = list.pop_tail_2();
        assert_eq!(value, Some(1));
        assert_eq!(list.count(), 0);
        assert!(list.tail.is_none());
        assert!(list.head.is_none());
    }

    #[test]
    fn test_pop_tail2_2() {
        let mut list = LinkedList::new();
        list.push_back_2(1);
        list.push_back_2(2);
        list.push_back_2(3);

        let value = list.pop_tail_2();
        assert_eq!(value, Some(3));

        let value = list.pop_tail_2();
        assert_eq!(value, Some(2));

        let value = list.pop_tail_2();
        assert_eq!(value, Some(1));
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_iterator() {
        let list = LinkedList::from(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ]);

        let mut iter = LinkedListIter::new(list.head());
        for i in 1..=10 {
            assert_eq!(iter.next(), Some(i));
        }

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let list = LinkedList::from(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ]);
        let mut i = 1;
        for item in list {
            assert_eq!(item, i);
            i += 1;
        }
    }
}
