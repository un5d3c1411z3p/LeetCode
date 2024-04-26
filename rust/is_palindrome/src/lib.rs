/*
9. Palindrome Number
Easy
Topics
Companies
Hint
Given an integer x, return true if x is a
palindrome
, and false otherwise.



Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.


Constraints:

-231 <= x <= 231 - 1

Follow up: Could you solve it without converting the integer to a string?
*/
struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let result = Self::reverse_digits(x);
        x == result
    }

    fn reverse_digits(x: i32) -> i32 {
        let mut result = 0;
        let mut copy = x;
        while copy != 0 {
            result = result * 10 + copy % 10;
            copy /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_palindrome(121);
        assert_eq!(result, true);
        let result = Solution::is_palindrome(-121);
        assert_eq!(result, false);
        let result = Solution::is_palindrome(10);
        assert_eq!(result, false);
    }
}
