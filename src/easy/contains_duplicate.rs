use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let size_of_set = HashSet::<i32>::from_iter(nums.clone());

    &nums.len() != &size_of_set.len()
}

#[test]
fn test_contains_duplicate() {
    assert_eq!(contains_duplicate(vec![0, 1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(vec![0, 1, 2, 3, 3]), true);
    assert_eq!(contains_duplicate(vec![0, 1, 2, 2, 3, 3]), true);
}
