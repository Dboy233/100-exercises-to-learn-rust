// TODO:每当通过其访问器方法返回 'title' 和 'description' 时， 它们应该被规范化，即应该删除前导和尾随的空格。
//   在 Rust 的标准库中有一个方法可以帮助解决这个问题，但你不会在 'String' 的文档中找到它。
//   您能弄清楚它的定义位置以及如何使用它吗？
//

pub struct Ticket {
    title: String,
    description: String,
    #[allow(unused)]
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        &self.title.trim()
    }

    pub fn description(&self) -> &str {
        &self.description.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
