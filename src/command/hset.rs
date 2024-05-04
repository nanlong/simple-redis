use anyhow::Result;

use super::{parse::Parse, CommandExecute};
use crate::backend::Backend;
use crate::resp::frame::Frame;

#[derive(Debug)]
pub struct HSet {
    key: String,
    field: String,
    value: Frame,
}

impl CommandExecute for HSet {
    fn execute(&self, backend: Backend) -> Result<Frame> {
        backend.hset(&self.key, &self.field, self.value.clone());
        Ok(1.into())
    }
}

impl TryFrom<Frame> for HSet {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "HSET" {
            anyhow::bail!("Invalid command");
        }

        let key = parse.next_string()?;
        let field = parse.next_string()?;
        let value = parse.next()?;
        parse.finish()?;

        Ok(Self { key, field, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hget_try_from_frame() {
        let frame: Frame = vec![
            b"hset".into(),
            b"key".into(),
            b"field".into(),
            b"value".into(),
        ]
        .into();

        let actual: HSet = frame.try_into().unwrap();

        let expected = HSet {
            key: "key".to_string(),
            field: "field".to_string(),
            value: b"value".into(),
        };

        assert_eq!(actual.key, expected.key);
        assert_eq!(actual.field, expected.field);
        assert_eq!(actual.value, expected.value);
    }
}
