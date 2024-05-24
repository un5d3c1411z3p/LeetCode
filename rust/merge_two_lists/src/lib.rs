/*
21. Merge Two Sorted Lists
Easy
Topics
Companies
You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.



Example 1:


Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
Example 2:

Input: list1 = [], list2 = []
Output: []
Example 3:

Input: list1 = [], list2 = [0]
Output: [0]


Constraints:

The number of nodes in both lists is in the range [0, 50].
-100 <= Node.val <= 100
Both list1 and list2 are sorted in non-decreasing order.
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = list1;
        let mut l2 = list2;

        let mut temporary = &mut result;
        while l2.is_some() {
            if temporary.is_none() || l2.as_ref()?.val < temporary.as_ref()?.val {
                std::mem::swap(temporary, &mut l2);
            }
            temporary = &mut temporary.as_mut()?.next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(
            result,
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 4, next: None }))
                            }))
                        }))
                    }))
                }))
            }))
        );

        let list1 = None;
        let list2 = None;
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, None);

        let list1 = None;
        let list2 = Some(Box::new(ListNode { val: 0, next: None }));
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, Some(Box::new(ListNode { val: 0, next: None })));
    }
}
