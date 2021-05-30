pub struct RadixSort;

impl RadixSort {
    pub fn radix_sort(arr: &mut [i32]) {
        if arr.is_empty() || arr.len() < 2 {
            return;
        }
        let max_bit = Self::max_bits(arr);
        Self::radix_sort_core(arr, 0, arr.len() - 1, max_bit);
    }

    fn max_bits(arr: &[i32]) -> usize {
        let mut max = i32::MIN;
        for &item in arr.iter() {
            max = max.max(item);
        }
        let mut res = 0;
        while max != 0 {
            res += 1;
            max /= 10;
        }
        res
    }

    fn radix_sort_core(arr: &mut [i32], left: usize, right: usize, digit: usize) {
        let radix = 10;
        // 有多少个数准备多少额外空间
        let mut bucket = vec![0; right - left + 1];
        for d in 1..=digit { // 有多少位就进出多少次
           // 十个空间
           // count[0] 当前/d位是0的数字有多少个
           // count[1] 当前/d位是0或1的数字有多少个
           // count[2] 当前/d位是0、1、或2的数字有多少个
           // count[i] 当前/d位是0||..||i的数字有多少个
            let mut count = vec![0; radix];
            for i in left..=right {
                let j = Self::get_digit(arr[i], d as u32);
                count[j] += 1;
            }
            for i in 1..radix {
                count[i] += count[i - 1];
            }
            let mut i = right as isize;
            // 对原数组反向遍历，确定其在bucket的位置
            while i >= left as isize {
                let j = Self::get_digit(arr[i as usize], d as u32);
                count[j] -= 1;
                bucket[count[j]] = arr[i as usize];
                i -= 1;
            }
            // 将辅助数组复制到原数组，完成一个数位的排序
            for (i, &item) in bucket.iter().enumerate() {
                arr[left + i] = item;
            }
        }
    }

    fn get_digit(num: i32, d: u32) -> usize {
        ((num / 10_i32.pow(d - 1)) % 10_i32) as usize
    }
}

#[test]
fn test() {
    let mut v = vec![123, 111, 23, 45, 654, 2];
    RadixSort::radix_sort(&mut v);
    assert_eq!(v, vec![2, 23, 45, 111, 123, 654]);
}