// TODO: 定义新的 'Order' 类型。
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   产品名称不能为空，并且不能超过 300 字节。
//   数量必须严格大于零。
//   单价以美分为单位，并且必须严格大于零。
//   Order 必须包含一个名为 'total' 的方法，该方法返回订单的总价。
//   Order 必须为每个字段提供 setter 和 getter。
//
// 这次测试位于不同的位置 — 在 'tests' 文件夹中。
// 'tests' 文件夹是 'cargo' 的特殊位置。这是它查找 **集成测试** 的地方。
// 这里的集成有一个非常具体的含义：他们测试您项目的 **公共 API**。
// 您需要注意类型和方法的可见性;集成
// 测试无法访问私有项目或 'pub（crate）' 项目。

pub struct Order {
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: u32,

}

impl Order {
    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &str {
        self.product_name.as_ref()
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }


    pub fn set_product_name(&mut self, name: String) {
        if (name.is_empty()) {
            panic!()
        }
        if name.len() > 300 {
            panic!()
        }
        self.product_name = name;
    }
    pub fn set_quantity(&mut self, q: u32) {
        if q == 0 {
            panic!()
        }
        self.quantity = q;
    }

    pub fn set_unit_price(&mut self, p: u32) {
        if p == 0 {
            panic!()
        }
        self.unit_price = p;
    }


    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        if product_name.is_empty() || product_name.len() > 300 || quantity == 0 || unit_price == 0 {
            panic!()
        }
        Self { product_name, quantity, unit_price }
    }
}
