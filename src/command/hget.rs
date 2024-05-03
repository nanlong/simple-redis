use anyhow::Result;

use super::parse::Parse;
use super::{CommandExecute, NULL};
use crate::resp::frame::Frame;
use crate::store::Store;

#[derive(Debug)]
pub struct HGet {
    key: String,
    field: String,
}

impl CommandExecute for HGet {
    fn execute(&self, store: Store) -> Result<Frame> {
        match store.hget(&self.key, &self.field) {
            Some(value) => Ok(value),
            None => Ok(NULL.clone()),
        }
    }
}

impl TryFrom<Frame> for HGet {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "HGET" {
            anyhow::bail!("Invalid command");
        }

        let key = parse.next_string()?;
        let field = parse.next_string()?;
        parse.finish()?;

        Ok(Self { key, field })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hget_try_from_frame() {
        let frame: Frame = vec!["hget".into(), "key".into(), "field".into()].into();

        let actual: HGet = frame.try_into().unwrap();
        let expected = HGet {
            key: "key".to_string(),
            field: "field".to_string(),
        };

        assert_eq!(actual.key, expected.key);
        assert_eq!(actual.field, expected.field);
    }

    #[test]
    fn test_hget_try_from_frame_invalid_command() {
        let frame: Frame = vec!["set".into(), "key".into(), "field".into()].into();

        let actual: Result<HGet> = frame.try_into();
        assert!(actual.is_err());
    }
}
