struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO:实现 'new' 函数。
    //  应满足以下要求：
    //   - 仅允许“To-Do”、“In Progress”和“Done”状态。
    //   - 'title' 和 'description' 字段不应为空。
    //   - 'title' 的长度最多应为 50 字节。
    //   - “description” 的长度应最多为 500 字节。
    //  如果未满足任何要求，该方法应 panic。
    //  您可以在测试中找到所需的 panic 消息。
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        match status.as_str() {
            "To-Do" | "In Progress" | "Done" => {}
            _ => { panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed") }
        }
        if (title.is_empty()) {
            panic!("Title cannot be empty");
        }
        if (description.is_empty()) {
            panic!("Description cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
