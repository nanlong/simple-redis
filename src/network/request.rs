use crate::command::{Command, CommandExecute};
use crate::resp::frame::Frame;
use crate::store::Store;
use anyhow::Result;

#[derive(Debug)]
pub struct RespRequest {
    command: Command,
    store: Store,
}

impl RespRequest {
    pub fn new(command: Command, store: Store) -> Self {
        Self { command, store }
    }

    pub fn execute(&self) -> Result<Frame> {
        self.command.execute(self.store.clone())
    }
}
