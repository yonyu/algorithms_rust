use std::collections::VecDeque;

mod arrays;
mod linked_lists;
/*
Topal: 2024/06/18 13:25
https://sharetask.io/t/gsxR1TZaqY4Zkdc8k0Xk0

A vending machine has the following denominations: 1c, 5c, 10c, 25c, 50c, and $1. Your task is to
write a program that will be used in a vending machine to return change. Assume that the vending
machine will always want to return the least number of coins or notes. Devise a function
getChange(M, P) where M is how much money was inserted into the machine and P the price of the item
selected, that returns an array of integers representing the number of each denomination to return.

Example:
getChange(5, 0.99) // should return [1,0,0,0,0,4]

getChange(3.14, 1.99) // should return [0,1,1,0,0,1]
getChange(3, 0.01) // should return [4,0,2,1,1,2]
getChange(4, 3.14) // should return [1,0,1,1,1,0]
getChange(0.45, 0.34) // should return [1,0,1,0,0,0]
 */

// M is the total money inserted into the vending machine
// P is the price of the item selected
// the result requires the coins with smaller value at the beginning
// the coin with the largest value at the end
fn getChange(M:f32, P: f32) -> VecDeque<i32> {
    // use a deque as we insert new element at the front
    let mut changes = VecDeque::new();

    let denominations = [1, 5, 10, 25, 50, 100];
    // remaining (the change that should be returned back to a customer) = Given_amount - spent
    let mut remaining = (M * 100.0 - P * 100.0).round() as i32;

    for &denomination in denominations.iter().rev() {
        let count = remaining / denomination;
        changes.push_front(count);

        remaining -= count * denomination;
    }
    changes
}


// M is the total money inserted into the vending machine
// P is the price of the item selected
// the result requires the coins with smaller value at the beginning
// the coin with largest value at the end
fn getChange2(M:f32, P: f32) -> VecDeque<i32>
{
    let mut changes = VecDeque::new();
    let denominations = [1,5,10,25,50,100];
    // remaining (the change that should be returned back to a customer) = Given_amount - spent
    let mut remaining = (M * 100.0 - P * 100.0).round() as i32;
    changes.push_front(0);
    for v in 1 .. 6 {
        let mut t = i32::MAX;
        let mut j = 1;
        while j <= denominations.len() && v - denominations[j-1] >= 0 {
            t = std::cmp::min(changes[v - denominations[j-1]], t);
            j += 1;
        }

        changes.push_front(t + 1);
    }
    changes
}

fn main() {

    let M  = 5.0;
    let P= 0.99;
    let f = getChange(M, P);
    println!("{:?}", f);

    let M  = 3.0;
    let P= 0.01;
    let f = getChange(M, P);
    println!("{:?}", f);
}

// fn getChange3(M: f32, P: f32) -> Vec<i32> {
//     let mut f = Vec::new();
//     let denom = [1,5,10,25,50,100];
//     f[0] = 0;
//     for i in 1 .. 6 {
//         let t = i32::max;
//         let j = 1;
//         while j && i -denom[j-1] >= 0 {
//             t = min( f[i - denom[j-1], t);
//             j += 1;
//         }
//         f[i] = t + 1;
//     }
//     f
// }

// fn getChange2(M: f32, p: f32, v: f32) -> Vec<i32> {
//     let mut f: Vec<i32> = Vec::new();
//     let denom = [1,5,10,25,50,100];
//     f[0] = 0;
//     let i = 1;
//     for {
//         let mut t = i32::max;
//         let mut j = 1;
//         while j <= denom.size() && i -denom[j-1] >= 0 {
//             if i - denom[j-1] {
//                 t = (i as i32) - denom[j - 1];
//             }
//
//             j += 1;
//         }
//         f[i] = t + 1;
//     }
//     f
// }