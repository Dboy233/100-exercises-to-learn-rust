fn compute(a: u32, b: u32) -> u32 {
    // TODO： 更改下面的行以修复编译器错误并使测试通过。
    a + b * 4u32
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
