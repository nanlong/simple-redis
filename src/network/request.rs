use crate::backend::Backend;
use crate::command::{Command, CommandExecute};
use crate::resp::frame::Frame;
use anyhow::Result;

#[derive(Debug)]
pub struct RespRequest {
    command: Command,
    backend: Backend,
}

impl RespRequest {
    pub fn new(command: Command, backend: Backend) -> Self {
        Self { command, backend }
    }

    pub fn execute(&self) -> Result<Frame> {
        self.command.execute(self.backend.clone())
    }
}
