/*
35. Search Insert Position
Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.



Example 1:

Input: nums = [1,3,5,6], target = 5
Output: 2
Example 2:

Input: nums = [1,3,5,6], target = 2
Output: 1
Example 3:

Input: nums = [1,3,5,6], target = 7
Output: 4


Constraints:

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums contains distinct values sorted in ascending order.
-104 <= target <= 104
 */
struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::search_insert([1, 3, 5, 6].to_vec(), 5);
        assert_eq!(result, 2);
        let result = Solution::search_insert([1, 3, 5, 6].to_vec(), 2);
        assert_eq!(result, 1);
        let result = Solution::search_insert([1, 3, 5, 6].to_vec(), 7);
        assert_eq!(result, 4);
    }
}
