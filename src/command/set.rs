use anyhow::Result;

use super::{parse::Parse, CommandExecute, OK};
use crate::resp::frame::Frame;
use crate::store::Store;

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Frame,
}

impl CommandExecute for Set {
    fn execute(&self, store: Store) -> Result<Frame> {
        store.set(self.key.clone(), self.value.clone());
        Ok(OK.clone())
    }
}

impl TryFrom<Frame> for Set {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "SET" {
            anyhow::bail!("Invalid command");
        }

        let key = parse.next_string()?;
        let value = parse.next()?;
        parse.finish()?;

        Ok(Self { key, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::frame::Frame;
    use std::convert::TryInto;

    #[test]
    fn test_set_try_from_frame() {
        let frame: Frame = vec!["set".into(), "key".into(), "value".into()].into();

        let actual: Set = frame.try_into().unwrap();
        let expected = Set {
            key: "key".to_string(),
            value: "value".into(),
        };

        assert_eq!(actual.key, expected.key);
        assert_eq!(actual.value, expected.value);
    }

    #[test]
    fn test_set_try_from_frame_invalid_command() {
        let frame: Frame = vec!["get".into(), "key".into(), "value".into()].into();

        let actual: Result<Set> = frame.try_into();
        assert!(actual.is_err());
    }

    #[test]
    fn test_set_try_from_frame_invalid_parts() {
        let frame: Frame = vec!["set".into(), "key".into()].into();

        let actual: Result<Set> = frame.try_into();
        assert!(actual.is_err());
    }
}
