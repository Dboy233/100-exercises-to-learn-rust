// 使用以下字段定义名为 'Order' 的结构体：
// - 'price'，一个无符号整数
// - 'quantity'，一个无符号整数
//
// 它还应该有一个名为 'is_available' 的方法，如果数量大于 0，则返回 'true'，否则返回 'false'。
//

struct Order{
    price:u32,
    quantity:u32,
}

impl Order{
    fn is_available(&self)->bool{
        if self.quantity>0{
            true
        }else{
            false
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
