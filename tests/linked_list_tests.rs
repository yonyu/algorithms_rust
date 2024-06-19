// integration tests require the lib.rs file exists
// and lib.rs has the used modules declared in it
//use algorithms_rust::*;
//use algorithms_rust::linked_lists::linked_list::LinkedList;

use std::collections::LinkedList;

#[test]
fn it_works() {
    let mut list = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn invert_array_copy() {
    let m1: [i32; 5] = [1, 2, 3, 4, 5];
    let m2: Vec<i32> = m1.iter().copied().rev().collect();

    assert_eq!(vec![5, 4, 3, 2, 1], m2);
}

#[test]
fn invert_array_move() {
    let m1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let m2: Vec<i32> = m1.into_iter().rev().collect();

    assert_eq!(vec![5, 4, 3, 2, 1], m2);
}