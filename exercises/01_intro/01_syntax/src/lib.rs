// TODO： 修复下面的函数签名以使测试通过。
// 请务必阅读编译器错误消息 — Rust 编译器是您的结对编程
// 与本课程合作，它通常会引导您朝着正确的方向前进！
//
// 输入参数应具有与返回类型相同的类型。
fn compute(a:u32, b:u32) -> u32 {
    // Don't touch the function body.
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}