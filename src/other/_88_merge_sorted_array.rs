struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        let mut index = nums1.len() as i32 - 1;
        while m > 0 && n > 0 {
            if nums1[m as usize - 1] >= nums2[n as usize - 1] {
                nums1[index as usize] = nums1[m as usize - 1];
                m -= 1;
                index -= 1;
            } else {
                nums1[index as usize] = nums2[n as usize - 1];
                n -= 1;
                index -= 1;
            }
        }
        while m > 0 {
            nums1[index as usize] = nums1[m as usize - 1];
            m -= 1;
            index -= 1;
        }
        while n > 0 {
            nums1[index as usize] = nums2[n as usize - 1];
            n -= 1;
            index -= 1;
        }
    }
}

#[test]
fn test() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
