pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        let first_param = 10;
        let second_param = 20;
        assert!(first_param < second_param);
    }

    #[test]
    fn you_can_assert_eq() {
        let expected = 10;
        let actual = 10;
        assert_eq!(expected, actual);
    }

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(7));
    }
}
