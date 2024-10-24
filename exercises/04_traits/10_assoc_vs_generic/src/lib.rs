// TODO: 定义一个新的特征 'Power'，它有一个方法 'power' ，将 'self' 提升到 'n' 的幂。
//  trait 定义及其实现应该足以使测试编译并通过。
//
// Recommendation: 您可能很想编写一个通用 implementation 来一次处理所有情况。然而
//  这相当复杂，需要使用额外的 crate（即 'num-traits'）。
//
//即便如此，使用简单的宏可能更可取，以避免高度通用实现的复杂性。
// 如果你有兴趣了解更多信息，请查看 “Little book of Rust macros” （https://veykril.github.io/tlborm/）。
// 不过你不必这样做：手动编写三个单独的 implementations 是完全可以的。只有在你好奇的情况下才进一步冒险。


// 定义 Power 特征
//

pub trait Power<Exponent = Self> {
    type Output;

    fn power(&self, n: Exponent) -> Self::Output;
}

impl Power<u16> for u32 {
    type Output = u32;

    fn power(&self, n: u16) -> Self::Output {
        self.pow(n.into())
    }
}

impl Power<&u32> for u32 {
    type Output = u32;

    fn power(&self, n: &u32) -> Self::Output {
        self.power(*n)
    }
}

impl Power<u32> for u32 {
    type Output = u32;

    fn power(&self, n: u32) -> Self::Output {
        self.pow(n)
    }
}


#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
