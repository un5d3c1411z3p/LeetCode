/*
66. Plus One
Easy

You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.

Increment the large integer by one and return the resulting array of digits.



Example 1:

Input: digits = [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Incrementing by one gives 123 + 1 = 124.
Thus, the result should be [1,2,4].
Example 2:

Input: digits = [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
Incrementing by one gives 4321 + 1 = 4322.
Thus, the result should be [4,3,2,2].
Example 3:

Input: digits = [9]
Output: [1,0]
Explanation: The array represents the integer 9.
Incrementing by one gives 9 + 1 = 10.
Thus, the result should be [1,0].


Constraints:

1 <= digits.length <= 100
0 <= digits[i] <= 9
digits does not contain any leading 0's.
 */
struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut r = digits.clone();

        println!("[Before] r is {:?}", r);

        let mut carry = 0;
        for e in r.iter_mut().rev() {
            if *e > 8 {
                *e = 0;
                carry = 1;
            } else {
                *e += 1;
                carry = 0;
                break;
            }
        }

        if carry == 1 {
            r.insert(0, 1);
        }

        println!("[After] r is {:?}", r);

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::plus_one([1, 2, 3].to_vec());
        assert_eq!(result, [1, 2, 4].to_vec());
        let result = Solution::plus_one([4, 3, 2, 1].to_vec());
        assert_eq!(result, [4, 3, 2, 2].to_vec());
        let result = Solution::plus_one([9].to_vec());
        assert_eq!(result, [1, 0].to_vec());
        let result = Solution::plus_one([4, 3, 2, 9].to_vec());
        assert_eq!(result, [4, 3, 3, 0].to_vec());
        let result = Solution::plus_one([9, 9].to_vec());
        assert_eq!(result, [1, 0, 0].to_vec());
    }
}
