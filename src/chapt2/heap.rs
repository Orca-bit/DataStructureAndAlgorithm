pub struct Heap;

impl Heap {
    pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
        if arr.is_empty() || arr.len() < 2 {
            return;
        }
//        for i in 0..arr.len() {             // 堆化
//            Self::heap_insert(arr, i);
//        }
        let mut n = arr.len() as isize - 1;
        while n >= 0 {
            Self::heapify(arr, n as usize, arr.len());
            n -= 1;
        }

        let mut heap_size = arr.len() - 1;
        arr.swap(0, heap_size);
        while heap_size > 0 {
            Self::heapify(arr, 0, heap_size);
            heap_size -= 1;
            arr.swap(0, heap_size);
        }

    }

    fn heapify<T: PartialOrd>(arr: &mut [T], index: usize, heap_size: usize) {
        let mut index = index;
        let mut left = index * 2 + 1;
        while left < heap_size {
            let mut largest = if left + 1 < heap_size && arr[left + 1] > arr[left] {
                left + 1
            } else {
                left
            };
            largest = if arr[largest] > arr[index] {
                largest
            } else {
                index
            };
            if largest == index {
                break;
            }
            arr.swap(index, largest);
            index = largest;
            left = index * 2 + 1;
        }
    }

    fn heap_insert<T: PartialOrd>(arr: &mut [T], index: usize) {
        let mut index = index as isize;
        while arr[index as usize] > arr[((index - 1) / 2) as usize] {
            arr.swap(index as usize, ((index - 1) / 2) as usize);
            index = (index - 1) / 2;
        }
    }
}

#[test]
fn test() {
    let mut arr = vec![8, 7, 6, 5, 4, 3, 2, 1];
    Heap::heap_sort(&mut arr);
    println!("{:?}",arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
