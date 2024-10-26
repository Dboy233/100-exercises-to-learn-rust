// TODO: 使用两个变体，一个用于标题错误，一个用于描述错误。
//   每个变体都应该包含一个字符串，并说明到底出了什么问题。
//   您还必须更新 `Ticket::new` 的实现。
#[derive(Debug, PartialEq, Clone)]
enum TicketNewError {
    TitleError(String),
    DescError(String),
}

// TODO: 当标题无效时，“easy_ticket”应该出现恐慌，使用存储在“TicketNewError”枚举的相关变体中的错误消息。
//   当描述无效时，应使用默认描述：“未提供描述”。
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description, status.clone()) {
        Ok(ticket) => { ticket }
        Err(error) => {
            match error {
                TicketNewError::TitleError(info) => {
                    panic!("{info}")
                }
                TicketNewError::DescError(_) => {
                    Ticket::new(title, "Description not provided".to_string(), status).unwrap()
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError("Title cannot be empty".to_string()));
        }
        if title.len() > 50 {
            return Err(TicketNewError::DescError("Title cannot be longer than 50 bytes".to_string()));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescError("Description cannot be empty".to_string()));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescError("Description cannot be longer than 500 bytes".to_string()));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
