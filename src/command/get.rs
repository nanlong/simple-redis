use anyhow::Result;

use super::parse::Parse;
use super::{CommandExecute, NULL};
use crate::resp::frame::Frame;
use crate::store::Store;

#[derive(Debug)]
pub struct Get {
    pub(crate) key: String,
}

impl CommandExecute for Get {
    fn execute(&self, store: Store) -> Result<Frame> {
        match store.get(&self.key) {
            Some(value) => Ok(value),
            None => Ok(NULL.clone()),
        }
    }
}

impl TryFrom<Frame> for Get {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "GET" {
            anyhow::bail!("Invalid command");
        }

        let key = parse.next_string()?;
        parse.finish()?;

        Ok(Self { key })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::frame::Frame;
    use std::convert::TryInto;

    #[test]
    fn test_get_try_from_frame() {
        let frame: Frame = vec!["get".into(), "key".into()].into();

        let actual: Get = frame.try_into().unwrap();
        let expected = Get {
            key: "key".to_string(),
        };

        assert_eq!(actual.key, expected.key);
    }

    #[test]
    fn test_get_try_from_frame_invalid_command() {
        let frame: Frame = vec!["set".into(), "key".into()].into();

        let actual: Result<Get> = frame.try_into();
        assert!(actual.is_err());
    }

    #[test]
    fn test_get_try_from_frame_invalid_parts() {
        let frame: Frame = vec!["set".into()].into();

        let actual: Result<Get> = frame.try_into();
        assert!(actual.is_err());
    }
}
