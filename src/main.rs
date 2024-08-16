#![allow(dead_code)]

mod arrays;
mod dynamic_programming;
mod grids;
mod linked_lists;
mod lists;
mod strings;
mod trees;

//use std::collections::VecDeque;

fn main() {
    println!("Hello world!");
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
