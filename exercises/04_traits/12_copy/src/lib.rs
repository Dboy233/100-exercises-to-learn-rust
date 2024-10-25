// TODO: 实现必要的特征以使测试编译并通过。
//  你 * 不能 * 修改测试。

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}


impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    type Output = WrappingU32;//相加操作执行完成之后返回的类型,这里是自身，以实现a+b+c

    fn add(self, rhs: Self) -> Self::Output {
        //wrapping_add函数是数字类型，如果相加超过了最大值，则
         WrappingU32::new(self.value.wrapping_add(rhs.value))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        println!("u32 Max {}", u32::MAX);
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
