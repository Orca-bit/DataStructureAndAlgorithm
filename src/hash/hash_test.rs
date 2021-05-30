#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn hashmap_test() {
        let mut hash1 = HashMap::new();
        hash1.insert(1, "one".to_string());
        hash1.insert(2, "two".to_string());
        hash1.insert(3, "three".to_string());
        assert_eq!(hash1.contains_key(&1), true);
        if let Some(value) = hash1.get(&1) {
            assert_eq!(*value, "one".to_string());
        }
        if let Some(value) = hash1.get_mut(&1) {
            *value = "new one".to_string();
            assert_eq!(*value, "new one".to_string());
        }
        hash1.remove(&1);
        assert_eq!(hash1.contains_key(&1), false);
        if let Some(value) = hash1.insert(2, "new two".to_string()) {
            assert_eq!(value, "two".to_string());
            let new_value = hash1.get(&2).unwrap();
            assert_eq!(*new_value, "new two".to_string());
        }
        hash1.clear();
        assert_eq!(hash1.is_empty(), true);
    }
}
