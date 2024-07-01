/*
69. Sqrt(x)
Easy
Hint
Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.

You must not use any built-in exponent function or operator.

For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.


Example 1:

Input: x = 4
Output: 2
Explanation: The square root of 4 is 2, so we return 2.
Example 2:

Input: x = 8
Output: 2
Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.


Constraints:

0 <= x <= 231 - 1
 */
use std::cmp::Ordering;

struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            // non-negative => 0 is allowed
            return 0;
        }

        let mut lower = 2;
        let mut higher = 46340.min(x / 2); // max possible root sqrt(i32::MAX)

        // Boundary conditions for optimization

        if x <= 3 {
            return 1;
        }

        if x >= higher * higher {
            return higher;
        }

        // Binary search the correct perfect square

        while higher - lower > 1 {
            let mid = (higher + lower) / 2;
            let pow = mid * mid;
            match pow.cmp(&x) {
                Ordering::Less => {
                    lower = mid;
                }
                Ordering::Greater => {
                    higher = mid;
                }
                _ => {
                    return mid;
                }
            }
        }

        // In the edge case that the number is between 2 values,
        // we take the lower which is equivalent to taking the floored mean
        lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::my_sqrt(4);
        assert_eq!(result, 2);
        let result = Solution::my_sqrt(8);
        assert_eq!(result, 2);
    }

    #[test]
    fn added_test_cases() {
        let result = Solution::my_sqrt(1);
        assert_eq!(result, 1);
        let result = Solution::my_sqrt(2147395600);
        assert_eq!(result, 46340);
    }
}
