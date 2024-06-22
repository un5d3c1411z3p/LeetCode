/*
58. Length of Last Word
Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal
substring
 consisting of non-space characters only.



Example 1:

Input: s = "Hello World"
Output: 5
Explanation: The last word is "World" with length 5.
Example 2:

Input: s = "   fly me   to   the moon  "
Output: 4
Explanation: The last word is "moon" with length 4.
Example 3:

Input: s = "luffy is still joyboy"
Output: 6
Explanation: The last word is "joyboy" with length 6.


Constraints:

1 <= s.length <= 104
s consists of only English letters and spaces ' '.
There will be at least one word in s.
 */
struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::length_of_last_word("Hello World".to_string());
        assert_eq!(result, 5);
        let result = Solution::length_of_last_word("   fly me   to   the moon  ".to_string());
        assert_eq!(result, 4);
        let result = Solution::length_of_last_word("luffy is still joyboy".to_string());
        assert_eq!(result, 6);
    }
}
