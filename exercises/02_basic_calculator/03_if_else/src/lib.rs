/// 如果 'n' 为偶数，则返回 'true'，否则返回 'false'。
fn is_even(n: u32) -> bool {
    if n % 2 == 0 {
        true
    }else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::is_even;

    #[test]
    fn one() {
        assert!(!is_even(1));
    }

    #[test]
    fn two() {
        assert!(is_even(2));
    }

    #[test]
    fn high() {
        assert!(!is_even(231));
    }
}
