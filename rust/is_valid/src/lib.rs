/*
20. Valid Parentheses
Easy
Topics
Companies
Hint
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.


Example 1:

Input: s = "()"
Output: true
Example 2:

Input: s = "()[]{}"
Output: true
Example 3:

Input: s = "(]"
Output: false


Constraints:

1 <= s.length <= 104
s consists of parentheses only '()[]{}'.
*/
struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_valid("()".to_string());
        assert_eq!(result, true);
        let result = Solution::is_valid("()[]{}".to_string());
        assert_eq!(result, true);
        let result = Solution::is_valid("(]".to_string());
        assert_eq!(result, false);
    }
}
