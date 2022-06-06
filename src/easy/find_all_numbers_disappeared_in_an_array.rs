use std::collections::HashSet;

pub fn find_all_numbers_disappeared_in_an_array(nums: Vec<i32>) -> Vec<i32> {
    let mut missing_nums = Vec::<i32>::new();
    let len = nums.len();
    let current_nums: HashSet<i32> = nums.into_iter().collect();

    for i in 1..=len {
        if !current_nums.contains(&i.try_into().unwrap()) {
            missing_nums.push(i as i32);
        }
    }

    missing_nums
}

#[test]
fn test_find_all_missing_numbers() {
    assert_eq!(
        vec![9],
        find_all_numbers_disappeared_in_an_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 10])
    );
    assert_eq!(
        vec![3],
        find_all_numbers_disappeared_in_an_array(vec![1, 2, 4])
    );
}
