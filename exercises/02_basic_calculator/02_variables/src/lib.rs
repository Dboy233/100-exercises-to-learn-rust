// 👇 以下以 '///' 开头的行称为 **文档注释**。
// 他们将文档附加到其后面的项目。在这种情况下，是 'speed' 功能。
// 如果你从这个练习的目录运行 'cargo doc --open'，Rust 将生成
// HTML 文档，并在浏览器中打开它。

/// 给定历程的起点和终点，以及完成历程所花费的时间，
/// 计算平均速度。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO： 定义一个名为 'distance' 的变量，并使用正确的值来使测试通过 你需要注释 'distance' 的类型吗？为什么或为什么不？
    let distance = end - start;
    // 不要更改下面的行
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
