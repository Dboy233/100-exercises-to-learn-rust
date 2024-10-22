///给定历程的起点和终点，以及完成历程所花费的时间，计算历程的平均速度。
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO： 如果 'time_elapsed' 为 0，则出现自定义消息的 panic
    if time_elapsed==0 {
        panic!("The journey took no time at all, that's impossible!")
    }
    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 使用 '#[should_panic]' 注解，我们可以断言我们预期被测代码会 panic。我们还可以使用 'expected' 来检查 panic 消息。
    //
    //    这都是 Rust 内置测试框架的一部分！
    #[should_panic(expected = "The journey took no time at all, that's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
