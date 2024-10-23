// 定义一个名为 'IsEven' 的 trait，该 trait 具有方法 'is_even' 如果 'self' 为 even，则返回 'true'，否则返回 'false'。
//
// Then implement the trait for `u32` and `i32`.

trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        if self % 2 == 0 {
            true
        } else {
            false
        }
    }
}
impl IsEven for i32 {
    fn is_even(&self) -> bool {
        if self % 2 == 0 {
            true
        }else {
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
