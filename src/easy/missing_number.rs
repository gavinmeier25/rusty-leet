pub fn missing_number(nums: Vec<i32>) -> i32 {
    (0..=nums.len() as i32)
        .chain(nums.into_iter())
        .fold(0, |acc, x| acc ^ x)
}

#[test]
fn test_missing_number() {
    assert_eq!(missing_number(vec![0, 2, 3, 4]), 1);
}
