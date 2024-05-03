use anyhow::Result;

use super::{parse::Parse, CommandExecute, NULL};
use crate::resp::frame::Frame;
use crate::store::Store;

#[derive(Debug)]
pub struct HGetAll {
    key: String,
}

impl CommandExecute for HGetAll {
    fn execute(&self, store: Store) -> Result<Frame> {
        let mut frame: Vec<Frame> = vec![];

        match store.hgetall(&self.key) {
            Some(hmap) => {
                for (field, value) in hmap {
                    frame.push(field.as_bytes().into());
                    frame.push(value);
                }
            }
            None => {
                return Ok(NULL.clone());
            }
        }

        Ok(frame.into())
    }
}

impl TryFrom<Frame> for HGetAll {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "HGETALL" {
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

    #[test]
    fn test_hgetall_try_from_frame() {
        let frame: Frame = vec!["hgetall".into(), "key".into()].into();

        let actual: HGetAll = frame.try_into().unwrap();
        let expected = HGetAll {
            key: "key".to_string(),
        };

        assert_eq!(actual.key, expected.key);
    }

    #[test]
    fn test_hgetall_try_from_frame_invalid_command() {
        let frame: Frame = vec!["set".into(), "key".into()].into();

        let actual: Result<HGetAll> = frame.try_into();
        assert!(actual.is_err());
    }
}
