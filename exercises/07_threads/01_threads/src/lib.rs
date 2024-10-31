// TODO: 使用 'spawn' 和 'join' 实现 'sum' 函数的多线程版本。
//  给定一个整数向量，将向量分成两半，并在单独的线程中对每一半求和。

// 警告：我们无法测试该功能的实现方式，
// 我们只能验证它是否产生了正确的结果。
// 你_可以_通过这个测试，只需返回 'v.iter（）.sum（）'，
// 但那样会违背演习的目的。
//
// 提示：你无法直接将生成的线程发送到 vector 的 _borrow_ slices。您需要分配新的
// 原始向量的每一半的向量。我们将在下一个练习中了解为什么这样做是必要的。
//
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let (v1,v2) = v.split_at(v.len() / 2);
    let yi_ban_1 = v1.to_vec();
    let yi_ban_2 = v2.to_vec();

    let handel_1 = thread::spawn(move || {
        yi_ban_1.iter().sum::<i32>()
    });
    let handel_2 =  thread::spawn(move || {
        yi_ban_2.iter().sum::<i32>()
    });
    let i1 = handel_1.join().unwrap();
    let i2 = handel_2.join().unwrap();
    i1 + i2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
