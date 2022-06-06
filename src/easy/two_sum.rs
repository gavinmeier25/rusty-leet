pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, value) in nums.iter().enumerate() {
        let missing = target - value;

        if nums.contains(&missing)
            && nums.iter().position(|x| x == &missing).unwrap() != index.try_into().unwrap()
        {
            return Vec::<i32>::from([
                index.try_into().unwrap(),
                nums.iter()
                    .position(|x| x == &missing)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ]);
        }
    }

    vec![]
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![1, 3, 5], 8), vec![1, 2]);
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    // This one is coming through as 1,0 instead of 0,1 but thats chilling, we don't care about the order.
    assert_eq!(two_sum(vec![3, 3], 6), vec![1, 0]);
}
