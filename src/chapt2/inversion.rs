struct Inversion;

impl Inversion {
    fn inversion(arr: &mut [i32]) -> i32 {
        if arr.is_empty() || arr.len() < 2 {
            return 0;
        }
        Self::process(arr, 0, arr.len() - 1)
    }

    fn process(arr: &mut [i32], left: usize, right: usize) -> i32 {
        if left == right {
            return 0;
        }
        let mid = left + ((right - left) >> 1);

        Self::process(arr, left, mid)
            + Self::process(arr, mid + 1, right)
            + Self::merge(arr, left, mid, right)
    }

    fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) -> i32 {
        let mut res = 0;
        let mut helper = vec![];
        let mut p1 = left;
        let mut p2 = mid + 1;
        while p1 <= mid && p2 <= right {
            if arr[p1] <= arr[p2] {
                helper.push(arr[p1]);
                p1 += 1;
            } else {
                res += (mid - p1 + 1) as i32;          //右区先压入说明右区当前数小，产生逆序对
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
        res
    }
}
#[test]
fn test() {
    assert_eq!(Inversion::inversion(&mut vec![2, 1, 1, 3, 2]), 3);
}
