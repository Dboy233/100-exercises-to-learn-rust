use crate::status::{ParseStatusError, Status};

// 我们已经在最早的练习之一中看到了如何声明模块，但我们还没有看到如何将它们提取到单独的文件中。
// 现在就来解决这个问题吧！
//
// 在最简单的情况下，当提取的模块是单个文件时，创建一个与模块同名的新文件并将模块内容移动到那里就足够了。
//
// 模块文件应与声明模块的文件放在同一目录中。
// 在这种情况下，'src/lib.rs'，因此 'status.rs' 应该放在 'src' 目录中。
mod status;

// TODO: 当状态字符串无效时，向 'TicketNewError' 添加新的错误变体。
//   当对该变体的错误调用 'source' 时，它应该返回 'ParseStatusError' 而不是 'None'。

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("{0}")]
    UnKnowError(#[source] ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: 将状态字符串解析为 'Status' 枚举。
        match Status::try_from(status) {
            Ok(status) => {
                Ok(Ticket {
                    title,
                    description,
                    status,
                })
            }
            Err(err) => {
                Err(TicketNewError::UnKnowError(err))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
