// TODO:给定一个整数向量，将其分成两半，并在单独的线程中计算每半之和。
//  不要执行任何堆分配。不要泄露任何内存。

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (v1, v2) = v.split_at(mid);
    thread::scope(|s| {
        let handle1 = s.spawn(|| {
            v1.iter().sum::<i32>()
        });
        let handle2 = s.spawn(|| {
            v2.iter().sum::<i32>()
        });
        handle1.join().unwrap() + handle2.join().unwrap()
    })
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
