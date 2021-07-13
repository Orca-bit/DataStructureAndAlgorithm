use super::util::*;

struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> TreeLink {
        let n = nums.len();
        match n {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode {
                val: nums[0],
                left: None,
                right: None,
            }))),
            size => {
                let mid = size >> 1;
                let left = nums[..mid].to_vec();
                let right = nums[mid + 1..].to_vec();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[mid],
                    left: Self::sorted_array_to_bst(left),
                    right: Self::sorted_array_to_bst(right),
                })))
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![-10, -3, 0, 5, 9];
    let bst = tree!(0, tree!(-3, tree!(-10), None), tree!(9, tree!(5), None));
    assert_eq!(Solution::sorted_array_to_bst(nums), bst);
}