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

use std::collections::VecDeque;
/*
 * Greedy Algorithm: this works for the US coin system, but not for some other coin systems.
 * It does not work for all denominations.
 * For example, M=12, P=6 for denomination [1, 3, 4], the answer should be [0, 2, 0].
 * This algorithm works by always taking the largest coin denomination possible. It does not
 * consider any smaller denominations.
 *
 * This function takes two parameters: M and P. M is the total amount of money inserted into the
 * vending machine, and P is the price of the item selected. The function returns an array of integers
 * representing the number of each denomination to return.
 */
// M is the total money inserted into the vending machine
// P is the price of the item selected
// the result requires the coins with smaller value at the beginning
// the coin with the largest value at the end
#[allow(dead_code)]
fn get_change(m:f32, p: f32) -> VecDeque<i32> {
    // use a deque as we insert new element at the front
    let mut changes = VecDeque::new();

    let denominations = [1, 5, 10, 25, 50, 100];
    // remaining (the change that should be returned back to a customer) = Given_amount - spent
    let mut remaining = (m * 100.0 - p * 100.0).round() as i32;

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
// the coin with the largest value at the end
#[allow(dead_code)]
fn get_change_2(_m:f32, _p: f32) -> VecDeque<i32>
{
    let mut changes = VecDeque::new();
    let denominations = [1,5,10,25,50,100];
    // remaining (the change that should be returned back to a customer) = Given_amount - spent
    //let remaining = (m * 100.0 - p * 100.0).round() as i32;
    changes.push_front(0);
    for v in 1 .. 6 {
        let mut t = i32::MAX;
        let mut j = 1;
        while j <= denominations.len() && v as i32 - denominations[j-1] as i32 >= 0 {
            t = std::cmp::min(changes[v - denominations[j-1]], t);
            j += 1;
        }

        changes.push_front(t + 1);
    }
    changes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_change() {
        assert_eq!(get_change(5.0, 0.99), [1,0,0,0,0,4]);
        assert_eq!(get_change(3.14, 1.99), [0,1,1,0,0,1]);
        assert_eq!(get_change(3.0, 0.01), [4,0,2,1,1,2]);
        assert_eq!(get_change(4.0, 3.14), [1,0,1,1,1,0]);
        assert_eq!(get_change(0.45, 0.34), [1,0,1,0,0,0]);
    }
}