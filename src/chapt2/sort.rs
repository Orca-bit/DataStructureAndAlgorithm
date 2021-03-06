use rand::Rng;

pub struct Sort;
pub struct MergeSort;
pub struct QuickSort;

impl Sort {
    pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
        if arr.is_empty() || arr.len() < 2 {
            return;
        }
        for i in 0..arr.len() - 1 {
            let mut min_index = i;
            for j in (i + 1)..arr.len() {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
    }

    pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
        if arr.is_empty() || arr.len() < 2 {
            return;
        }
        let mut n = arr.len() - 1;
        while n > 0 {
            for i in 0..n {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                }
            }
            n -= 1;
        }
    }

    pub fn insert_sort<T: PartialOrd>(arr: &mut [T]) {
        if arr.is_empty() || arr.len() < 2 {
            return;
        }
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 {
                if arr[j - 1] > arr[j] {
                    arr.swap(j - 1, j);
                } else {
                    break;
                }
                j -= 1;
            }
        }
    }
}

impl MergeSort {
    pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        if arr.is_empty() || arr.len() < 2 {
            return;
        }
        Self::process(arr, 0, arr.len() - 1);
    }

    fn process<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) {
        if left == right {
            return;
        }
        let mid = left + ((right - left) >> 1);
        Self::process(arr, left, mid);
        Self::process(arr, mid + 1, right);
        Self::merge(arr, left, mid, right);
    }
    
    fn merge<T: PartialOrd + Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
        let mut helper = vec![];
        let mut p1 = left;
        let mut p2 = mid + 1;
        while p1 <= mid && p2 <= right {
           if arr[p1] <= arr[p2] {
               helper.push(arr[p1]);
               p1 += 1;
           } else {
               helper.push(arr[p2]);
               p2 += 1;
           }
        }
        while p1 <= mid {
            helper.push(arr[p1]);
            p1 += 1;
        }
        while p2 <= right {
            helper.push(arr[p2]);
            p2 += 1;
        }
        for (i, &item) in helper.iter().enumerate() {
            arr[left + i] = item;
        }
    }
}

impl QuickSort { // ??????????????????????????????usize????????????????????????isize
    pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
        if arr.is_empty() || arr.len() < 2 {
            return;
        }
        Self::process(arr, 0, (arr.len() - 1) as isize);
    }

    fn process<T: PartialOrd>(arr: &mut [T], left: isize, right: isize) {
        if left < right {
            let mut rng = rand::thread_rng();
            let random = rng.gen_range(left..=right);
            arr.swap(random as usize, right as usize);
            let (mid_left, mid_right) = Self::partition(arr, left, right);
            Self::process(arr, left, mid_left - 1);
            Self::process(arr, mid_right + 1, right);
        }
    }

    // ?????????right???????????????????????????=???????????????
    fn partition<T: PartialOrd>(arr: &mut [T], left: isize, right: isize) -> (isize, isize) {
        let mut p = left as usize; // p??????left--right???????????????????????????
        let mut less = left - 1; // ??????????????????????????????
        let mut more = right as usize; // ?????????????????????????????????right?????????????????????
        while p < more {
            if arr[p] < arr[right as usize] { // ????????????????????????????????????
                less += 1;
                arr.swap(less as usize, p);
                p += 1;
            } else if arr[p] > arr[right as usize] { // ????????????????????????????????????
                more -= 1;
                arr.swap(more, p);
            } else { //????????????????????????????????????
                p += 1;
            }
        }
        arr.swap(more, right as usize); // ??????????????????????????????????????????????????????more?????????????????????
        (less + 1, more as isize) // ?????????????????????
    }
}

#[test]
fn test() {
    // let mut arr = vec!['b', 'c', 'a', 'r'];
    // let mut arr2:Vec<i32> = vec![];
    // Sort::selection_sort(&mut arr2);
    // Sort::bubble_sort(&mut arr);
    // println!("{:?}",arr);
    // println!("{:?}",arr2);
    let mut arr = vec![8, 7, 6, 5, 4, 3, 2, 1];
    // Sort::selection_sort(&mut arr);
    QuickSort::quick_sort(&mut arr);
    println!("{:?}",arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
