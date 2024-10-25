// TODO: 添加必要的 'Clone' 实现 （和调用） 以获取要编译的代码。
//

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let t = ticket.clone();
    (ticket, t.summary())
}
#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}
#[derive(Clone)]
pub struct Summary {
    pub title: String,
    pub status: String,
}
