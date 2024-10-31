// TODO: 当您认为已完成此练习时，将 'move_forward' 设置为 'ready' 中的 'true'。
//  请随时致电讲师以验证您的解决方案！
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // 如果线程不再运行，这将 panic，因为通道将被关闭。
        .expect("你真的生成了一个线程吗？频道已关闭！");

}

#[test]
fn ready() {
    // 在这个练习中，我们可以自动检查的内容很少，
    // 因为我们的服务器没有公开任何 **read** 操作。
    // 我们无法知道插入是否真的发生，以及它们是否正确发生。
    let move_forward = true;

    assert!(move_forward);
}
