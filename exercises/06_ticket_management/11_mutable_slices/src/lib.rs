// TODO: 定义一个名为 'squared' 的函数，它将切片中的所有 'i32' 提高到 2 的幂次方。应就地修改切片。

fn squared(v: &mut [u32]) {
    for x in v.iter_mut() {
        *x *= *x
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = vec![];
        squared(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn one() {
        let mut s = [2];
        squared(&mut s);
        assert_eq!(s, [4]);
    }

    #[test]
    fn multiple() {
        let mut s = vec![2, 4];
        squared(&mut s);
        assert_eq!(s, vec![4, 16]);
    }
}
