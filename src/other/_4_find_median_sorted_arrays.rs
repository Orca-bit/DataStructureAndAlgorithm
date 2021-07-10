struct Solution;

impl Solution {
    pub fn find_median_sorted_array(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        let even = len & 1 == 0;
        if !nums1.is_empty() && !nums2.is_empty() {
            if even {
                (Self::find_kth_num(&nums1, &nums2, len / 2) as f64
                    + Self::find_kth_num(&nums1, &nums2, len / 2 + 1) as f64)
                    / 2.
            } else {
                Self::find_kth_num(&nums1, &nums2, len / 2 + 1) as f64
            }
        } else if !nums1.is_empty() {
            if even {
                (nums1[(len - 1) >> 1] + nums1[len >> 1]) as f64 / 2.
            } else {
                nums1[len >> 1] as f64
            }
        } else {
            if even {
                (nums2[(len - 1) >> 1] + nums2[len >> 1]) as f64 / 2.
            } else {
                nums2[len >> 1] as f64
            }
        }
    }

    fn find_kth_num(arr1: &[i32], arr2: &[i32], k: usize) -> i32 {
        let longs = if arr1.len() >= arr2.len() { arr1 } else { arr2 };
        let shorts = if arr1.len() < arr2.len() { arr1 } else { arr2 };
        if k <= shorts.len() {
            return Self::up_median(&shorts[0..k], &longs[0..k]);
        }
        if k > longs.len() {
            if shorts[k - longs.len() - 1] >= *longs.last().unwrap() {
                return shorts[k - longs.len() - 1];
            }
            if longs[k - shorts.len() - 1] >= *shorts.last().unwrap() {
                return longs[k - shorts.len() - 1];
            }
            return Self::up_median(&shorts[k - longs.len()..], &longs[k - shorts.len()..]);
        }
        if longs[k - shorts.len() - 1] >= *shorts.last().unwrap() {
            return longs[k - shorts.len() - 1];
        }
        Self::up_median(shorts, &longs[k - shorts.len()..k])
    }

    fn up_median(arr1: &[i32], arr2: &[i32]) -> i32 {
        assert_eq!(arr1.len(), arr2.len());
        let mut s1 = 0;
        let mut s2 = 0;
        let mut e1 = arr1.len() - 1;
        let mut e2 = arr2.len() - 1;
        while s1 < e1 {
            let mid1 = (s1 + e1) >> 1;
            let mid2 = (s2 + e2) >> 1;
            let offset = ((e1 - s1 + 1) & 1) ^ 1;
            if arr1[mid1] > arr2[mid2] {
                e1 = mid1;
                s2 = mid2 + offset;
            } else if arr1[mid1] < arr2[mid2] {
                s1 = mid1 + offset;
                e2 = mid2;
            } else {
                return arr1[mid1];
            }
        }
        arr1[s1].min(arr2[s2])
    }
}

#[test]
fn test() {
    assert_eq!(Solution::up_median(&vec![1, 2, 3, 4], &vec![3, 4, 5, 6]), 3);
    assert_eq!(Solution::up_median(&vec![0, 1, 2], &vec![3, 4, 5]), 2);
    assert_eq!(
        Solution::find_kth_num(&vec![1, 2, 3, 4, 5], &vec![3, 4, 5], 1),
        1
    );
    assert_eq!(Solution::find_kth_num(&vec![3, 4], &vec![1, 2, 5, 6], 3), 3);
    assert_eq!(Solution::find_kth_num(&vec![3, 4], &vec![1, 2, 5, 6], 4), 4);
    assert_eq!(Solution::find_median_sorted_array(vec![1, 3], vec![2]), 2.);
    assert_eq!(
        Solution::find_median_sorted_array(vec![3, 4], vec![1, 2, 5, 6]),
        3.5
    );
}
