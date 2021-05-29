pub mod chapt2;

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand::distributions::Uniform;
    // use super::chapt2::sort::Sort;
    // use super::chapt2::sort::MergeSort;
    use super::chapt2::sort::QuickSort;

    fn generate_random_item(max_size: i32, max_value: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(0..=max_size);
        let range = Uniform::new(0, max_value);

        let case: Vec<i32> = (0..size).map(|_| rng.sample(&range)).collect();
        case
    }

    #[test]
    fn it_works() {
        let max_size = 100;
        let max_value = 90;
        println!("{:?}",generate_random_item(max_size, max_value));
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sort_test() {
        let test_times = 500_000;
        let max_size = 100;
        let max_value = 100;
        for _ in 0..test_times {
            let mut arr1 = generate_random_item(max_size, max_value);
            let mut arr2 = arr1.clone();
            QuickSort::quick_sort(&mut arr1);
            arr2.sort();
            assert_eq!(arr1, arr2);
        }
    }
}
