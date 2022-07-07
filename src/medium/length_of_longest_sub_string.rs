pub fn length_of_longest_substring(s: String) -> i32 {
    let mut current_values = vec![];
    let mut highest_repeating_string = 0;

    for x in s.split_terminator("").into_iter() {
        if x.is_empty() {
            continue;
        };
        match current_values.iter().position(|y| y == &x) {
            Some(i) => {
                if highest_repeating_string < current_values.len() {
                    highest_repeating_string = current_values.len();
                }

                current_values = current_values.drain(i + 1..).collect();

                current_values.push(x);
            }
            None => {
                current_values.push(x);
                if highest_repeating_string < current_values.len() {
                    highest_repeating_string = current_values.len();
                }
            }
        }
    }

    highest_repeating_string.try_into().unwrap()
}

#[test]
fn test_add_two_numbers() {
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
}
