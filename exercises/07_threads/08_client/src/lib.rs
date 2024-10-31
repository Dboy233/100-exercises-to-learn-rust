use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{channel, Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: 充实客户端实现。
pub struct TicketStoreClient {
    server_sender: Sender<Command>,
}

impl TicketStoreClient {
    pub fn new(server_sender: Sender<Command>) -> Self {
        TicketStoreClient {
            server_sender,
        }
    }

    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (s, r) = channel();
        self.server_sender.send(Command::Insert {
            draft,
            response_channel: s.clone(),
        }).unwrap();
        r.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (s, r) = channel();
        self.server_sender.send(Command::Get {
            id,
            response_channel: s.clone(),
        }).unwrap();
        r.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient::new(sender)
}

// No longer public! This becomes an internal detail of the library now.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                   draft,
                   response_channel,
               }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                   id,
                   response_channel,
               }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
