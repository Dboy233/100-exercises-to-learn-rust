// TODO: Implement the patching functionality.
use crate::data::{Ticket, TicketDraft, TicketPatch};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn update(&self, ticket_patch: TicketPatch) -> Result<Ticket, UpdateError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender.try_send(
            Command::Update {
                patch: ticket_patch,
                response_channel: response_sender,
            }
        ).map_err(|_| UpdateError {
            msg: String::from("尝试更新失败")
        })?;
        response_receiver.recv().unwrap()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

#[derive(Debug, thiserror::Error)]
#[error("更新错误:{msg}")]
///更新错误
pub struct UpdateError {
    msg: String,
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
    Update {
        patch: TicketPatch,
        response_channel: SyncSender<Result<Ticket, UpdateError>>,
    },
}


pub fn server(receiver: Receiver<Command>) {
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
            Ok(Command::Update {
                   patch,
                   response_channel,
               }) => {
                let option = store.get_mut(patch.id);
                let ticket = match option {
                    None => {
                        response_channel.send(Err(UpdateError { msg: String::from("没有查找到对应id的Ticket") })).unwrap();
                        continue;
                    }
                    Some(tricket) => {
                        tricket
                    }
                };
                if let Some(status) = patch.status {
                    ticket.status = status;
                }
                if let Some(title) = patch.title {
                    ticket.title = title;
                }
                if let Some(des) = patch.description {
                    ticket.description = des;
                }
                response_channel.send(Ok(ticket.clone())).unwrap()
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
