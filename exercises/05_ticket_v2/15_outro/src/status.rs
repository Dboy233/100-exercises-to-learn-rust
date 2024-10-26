use crate::Status::{Done, InProgress, ToDo};

// TODO: <String>为 'Status' 枚举实现 'TryFrom' 和 'TryFrom<&str>' 。
//  解析应不区分大小写。
#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq, Clone, thiserror::Error)]
#[error("无法解析{msg}为Status")]
pub struct ParsStatusError {
    msg: String,
}

impl TryFrom<&str> for Status {
    type Error = ParsStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => { Ok(ToDo) }
            "inprogress" => { Ok(InProgress) }
            "done" => { Ok(Done) }
            &_ => { Err(ParsStatusError { msg: value.to_owned() }) }
        }
    }
}

impl TryFrom<String> for Status {
    type Error = ParsStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
         value.as_str().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
