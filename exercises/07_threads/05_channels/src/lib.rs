use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(data::TicketDraft),
}

// 通过生成服务器线程来启动系统。
// 它返回一个 'Sender' 实例，然后一个或多个客户端可以使用该实例与服务器交互。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: 服务器任务应该 **永不** 停止。
//  进入一个循环：等待命令出现在通道中，然后执行它，然后开始等待下一个命令。
pub fn server(receiver: Receiver<Command>) {
    let mut ticket_store = store::TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(command) => {
                match command {
                    Command::Insert(data) => {
                        let id = ticket_store.add_ticket(data);
                        println!("添加数据id {:?}", id);
                    }
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
