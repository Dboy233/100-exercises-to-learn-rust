//给定一个数字 'n'，返回斐波那契数列中的第 'n+1' 个数字。
//
// 斐波那契数列定义如下：
//
// - 序列的第一个数字是 0。
// - 序列的第二个数字是 1。
// - 每个后续数字都是前两个数字的总和。
//
// 所以顺序是：0、1、1、2、3、5、8、13、21 等。
//
// 我们期望 'fibonacci（0）' 返回 '0'，'fibonacci（1）' 返回 '1'，
// 'fibonacci（2）' 返回 '1'，依此类推。
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: 使用 'Vec' 来记住您已经计算过的结果，这样您就不必多次重新计算它们。
    let n = n as usize;
    let mut vec= vec![0,1];
    for i in 2..=n {
        vec.push(vec[i - 2] + vec[i - 1]);
    }
    vec[n]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
