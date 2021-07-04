struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut dp = vec![vec![0; m]; n];
        for j in 0..m {
            if nums1[0] == nums2[j] {
                dp[0][j] = 1;
            }
        }
        for i in 0..n {
            if nums1[i] == nums2[0] {
                dp[i][0] = 1;
            }
        }
        let mut res = 0;
        for i in 1..n {
            for j in 1..m {
                if nums1[i] == nums2[j] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    res = res.max(dp[i][j]);
                }
            }
        }
        res
    }

    fn find_length_1(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // 空间压缩dp
        let n = nums1.len();
        let m = nums2.len();
        let mut row = 0;
        let mut col = m - 1;
        let mut res = 0;
        while row < n {
            let mut i = row;
            let mut j = col;
            let mut len = 0;
            while i < n && j < m {
                if nums1[i] != nums2[j] {
                    len = 0;
                } else {
                    len += 1;
                }
                if res < len {
                    res = len;
                }
                i += 1;
                j += 1;
            }
            if col > 0 {
                col -= 1;
            } else {
                row += 1;
            }
        }
        res
    }

    fn find_length_2(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // 滑动窗口
        let n = nums1.len();
        let m = nums2.len();
        let max_length = |p1: usize, p2: usize, len: usize| -> i32 {
            let mut res = 0;
            let mut tmp = 0;
            for i in 0..len {
                if nums1[p1 + i] == nums2[p2 + i] {
                    tmp += 1;
                    res = res.max(tmp);
                } else {
                    tmp = 0;
                }
            }
            res
        };
        let mut res = 0;
        for i in 0..n {
            res = res.max(max_length(i, 0, m.min(n - i)));
        }
        for i in 0..m {
            res = res.max(max_length(0, i, n.min(m - i)));
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 2, 1];
    let b = vec![3, 2, 1, 4, 7];
    let res = 3;
    assert_eq!(Solution::find_length(a, b), res);
    let a = vec![1, 2, 3, 2, 1];
    let b = vec![3, 2, 1, 4, 7];
    let res = 3;
    assert_eq!(Solution::find_length_1(a, b), res);
    let a = vec![1, 2, 3, 2, 1];
    let b = vec![3, 2, 1, 4, 7];
    let res = 3;
    assert_eq!(Solution::find_length_2(a, b), res);
}
