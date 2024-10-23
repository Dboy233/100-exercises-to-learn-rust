pub struct Ticket {
    title: String, //24
    description: String, //24
    status: String, //24
}

// TODO: 根据您在本节中学到的知识，将 'TODO！（）' 替换为相应类型的正确 **堆栈大小**。
//
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        assert_eq!(size_of::<Ticket>(), 24 + 24 + 24);
    }
}
