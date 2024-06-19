use std::fmt::Debug;
//use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::linked_lists::linked_list_node::{Node, NodeLink};

#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    pub head: NodeLink<T>,
    pub count: i32,
}

impl <T: Clone> LinkedList<T> { // where T is Clone and Debug:  Clone + Debug
    pub fn new() -> Self {
        LinkedList {
            head: None,
            count: 0,
        }
    }

    // add a value to the front of the linked list
    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        new_node.borrow_mut().next = self.head.take();

        self.head = Some(new_node);
        self.count += 1;
    }

    // return the value of the first node in the linked list without removing it
    pub fn peek_front(&mut self) -> Option<T> {
        self.head.take().map(|node| Rc::try_unwrap(node).ok().unwrap().into_inner().value)
    }

    // return the first node in the linked list without removing it
    pub fn peek_front_node(&mut self) -> Option<Node<T>> {
        self.head.take().map(|node| Rc::try_unwrap(node).ok().unwrap().into_inner())
    }

    // remove and return the value of the first node in the linked list
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

    // return the value of the last node in the linked list without removing it
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

    // remove and return the value of the last node in the linked list
    pub fn pop_tail(&mut self) -> Option<T> {
        let mut current_link = self.head.clone();
        let mut previous_link: NodeLink<T> = None;
        while let Some(current_node) = current_link {
            let mut node = current_node.borrow();
            if node.clone().next.is_none() { // we have reached the end of the list
                previous_link.take().map(|mut n| n.borrow_mut().next = None); // sets the next node of the previous link to None
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

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_tail()
    }

    // add a value to the end of the linked list
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

    pub fn push_back(&mut self, value: T) { // O(n)
        self.push(value);
    }
}

pub struct ListIterator<T> {
    current: NodeLink<T>,
}

impl<T: Clone> ListIterator<T>{
    fn new(start_at: NodeLink<T>) -> ListIterator<T> {
        ListIterator {
            current: start_at,
        }
    }
}
impl <T: Clone> Iterator for ListIterator<T> {
    type Item = T;
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
        let mut list = LinkedList::new();
        list.push_front(1);
        println!("LIST: {:?}", list);
        println!("HEAD: {:?}", list.head);
        println!("COUNT: {:?}", list.count);
    }

    #[test]
    fn test_push_front2() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        println!("LIST: {:?}", list);
        println!("HEAD: {:?}", list.head);
        println!("COUNT: {:?}", list.count);
    }

    #[test]
    fn test_peek_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);

        let value = list.peek_front();
        println!("PEEK: {:?}", value);
    }

    #[test]
    fn test_peek_front_node() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);

        let value = list.peek_front_node();
        println!("PEEK: {:?}", value);
    }

    #[test]
    fn test_peek_tail() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let value = list.peek_tail();
        println!("PEEK: {:?}", value);
    }
    #[test]
    fn test_traverse() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let values = list.traverse();

        //println!("traversal: {:?}", values);
        assert_eq!(values, vec![3, 2, 1]);
    }

    #[test]
    fn test_pop_tail() {
        let mut list = LinkedList::new();
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
        assert_eq!(list.count, 0);
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
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
        assert_eq!(list.count, 0);
    }
}
