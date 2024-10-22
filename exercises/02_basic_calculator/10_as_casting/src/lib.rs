// TODO: 根据您在本节中学到的知识，将 'TODO！（）' 替换为正确的值。
//

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // 编译器足够聪明，知道值 255 不能放在 i8 中，
        //  所以它会发出一个硬错误。我们有意禁用此护栏，以便进行这种 （Bad） 转换。
        // 编译器只能选择 this，因为该值是 literals。
        // 如果我们要使用变量，编译器将无法在编译时捕获此变量。
        #[allow(overflowing_literals)]
        let x:i8 = { 255 as i8 };

        // 您可以使用与上述完全相同的表达式来解决此问题，但这会违背练习的目的。相反
        //  使用真正的 'i8' 值，该值在从 'u8' 转换时等效于 '255'。
        #[allow(overflowing_literals)]
        let y: i8 = { 255 as i8 };;

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
