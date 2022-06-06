pub fn palindrome_number(x: i32) -> bool {
    x.to_string().rsplit("").eq(x.to_string().split(""))
}

#[test]
fn test_palindrome_number() {
    assert_eq!(palindrome_number(1221), true);
    assert_eq!(palindrome_number(1231), false);
    assert_eq!(palindrome_number(1), true);
}
