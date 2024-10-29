fn is_palindrome(num: u32) -> bool {
    let original = num.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let cases = [
            (121, true),
            (123, false),
            (0, true),
            (11, true),
            (10, false),
            (12321, true),
        ];

        for (num, expected) in cases {
            assert_eq!(is_palindrome(num), expected);
        }
    }
}
