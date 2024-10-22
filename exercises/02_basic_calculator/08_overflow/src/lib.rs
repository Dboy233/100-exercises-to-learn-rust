// 自定义 'dev' 配置文件以在溢出时回绕。
// 查看 Cargo 的文档以找出正确的语法：
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// 出于我们稍后将解释的原因，自定义需要在存储库根目录的 'Cargo.toml' 中完成，而不是在练习的 'Cargo.toml' 中完成。
pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20!2432902008176640000，这太大了，无法放入 U32 中
        // 使用默认的 dev 配置文件，当你运行 'cargo test' 时，这将 panic
        // 我们希望它环绕
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // 使用下划线的大数字文本来提高可读性！
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
