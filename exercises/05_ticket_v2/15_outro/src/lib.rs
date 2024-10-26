// TODO: 你在这个箱子的每个模块中都有事情要做！
mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// and then re-export the public parts of those modules at the root of the crate.
//
// This hides the internal structure of the crate from your users, while still
// allowing you to organize your code however you like.
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// 我们不再需要将字段设为私有！
// 由于每个字段都封装了自己的验证逻辑，因此不存在 'Ticket' 的用户修改字段，以破坏结构的 invariants。
// 但要小心：如果您有任何跨多个字段的不变量，则需要确保这些不变量仍然被维护并返回将字段设为私有。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
