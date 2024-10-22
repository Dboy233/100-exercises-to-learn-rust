// 定义一个名为 'factorial' 的函数，给定一个非负整数 'n'，返回 'n！'，即 'n' 的阶乘。
//
// 'n' 的阶乘定义为 'n' 之前的所有正整数的乘积。
// 例如，'5！'（读作 “five factorial”）是 '5 * 4 * 3 * 2 * 1'，即 '120'。
// '0！' 定义为 '1'。
//
// 我们期望 'factorial（0）' 返回 '1'，'factorial（1）' 返回 '1'，'factorial（2）' 返回 '2'，依此类推。
//
// 只使用您学到的知识！还没有循环，所以你必须使用递归！

fn factorial(n: u32) -> u32 {
    if (n == 0) {
        1
    } else {
        n * factorial(n-1)
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

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
