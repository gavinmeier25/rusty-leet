pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |prev, cur| prev ^ cur)
}

#[test]
fn test_single_number() {
    assert_eq!(2, single_number(vec![1, 3, 1, 3, 2]));
    assert_eq!(9, single_number(vec![1, 3, 1, 3, 5, 5, 9]));
    assert_eq!(1, single_number(vec![1]));
}
