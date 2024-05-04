mod echo;
mod get;
mod hget;
mod hgetall;
mod hmget;
mod hset;
mod parse;
mod sadd;
mod set;
mod sismember;
mod smembers;

use crate::backend::Backend;
use crate::resp::frame::Frame;
use crate::resp::null::Null;
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use parse::Parse;

lazy_static! {
    static ref OK: Frame = b"OK".into();
    static ref NULL: Frame = Frame::Null(Null);
}

#[enum_dispatch]
pub trait CommandExecute {
    fn execute(&self, backend: Backend) -> Result<Frame>;
}

#[enum_dispatch(CommandExecute)]
#[derive(Debug)]
pub enum Command {
    Get(get::Get),
    Set(set::Set),
    HGet(hget::HGet),
    HSet(hset::HSet),
    HGetAll(hgetall::HGetAll),
    Echo(echo::Echo),
    Hmget(hmget::Hmget),
    Sadd(sadd::Sadd),
    Smembers(smembers::Smembers),
    Sismember(sismember::Sismember),
}

impl TryFrom<Frame> for Command {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame.clone())?;

        match parse.peek_string() {
            Ok(command) => match command.to_uppercase().as_str() {
                "GET" => Ok(Command::Get(frame.try_into()?)),
                "SET" => Ok(Command::Set(frame.try_into()?)),
                "HGET" => Ok(Command::HGet(frame.try_into()?)),
                "HSET" => Ok(Command::HSet(frame.try_into()?)),
                "HGETALL" => Ok(Command::HGetAll(frame.try_into()?)),
                "ECHO" => Ok(Command::Echo(frame.try_into()?)),
                "HMGET" => Ok(Command::Hmget(frame.try_into()?)),
                "SADD" => Ok(Command::Sadd(frame.try_into()?)),
                "SMEMBERS" => Ok(Command::Smembers(frame.try_into()?)),
                "SISMEMBER" => Ok(Command::Sismember(frame.try_into()?)),
                _ => anyhow::bail!("Invalid command"),
            },
            Err(_) => anyhow::bail!("Invalid command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::frame::Frame;
    use std::convert::TryInto;

    #[test]
    fn test_command_try_from_frame() {
        let frame: Frame = vec!["get".into(), "key".into()].into();

        let actual: Command = frame.try_into().unwrap();
        let expected = get::Get {
            key: "key".to_string(),
        };

        match actual {
            Command::Get(actual) => {
                assert_eq!(actual.key, expected.key);
            }
            _ => panic!("Expected Get"),
        }
    }

    #[test]
    fn test_command_get_and_set() {
        let backend = Backend::new();

        let frame: Frame = vec![b"get".into(), b"key".into()].into();
        let get_command: Command = frame.try_into().unwrap();

        let actual = get_command.execute(backend.clone()).unwrap();
        assert_eq!(actual, *NULL);

        let frame: Frame = vec![b"set".into(), b"key".into(), b"value".into()].into();
        let set_command: Command = frame.try_into().unwrap();

        let actual = set_command.execute(backend.clone()).unwrap();
        assert_eq!(actual, *OK);

        let result = get_command.execute(backend.clone()).unwrap();
        assert_eq!(result, b"value".into());
    }

    #[test]
    fn test_command_hget_and_hset() {
        let backend = Backend::new();

        let frame: Frame = vec![b"hget".into(), b"key".into(), b"field".into()].into();
        let hget_command: Command = frame.try_into().unwrap();

        let actual = hget_command.execute(backend.clone()).unwrap();
        assert_eq!(actual, *NULL);

        let frame: Frame = vec![
            b"hset".into(),
            b"key".into(),
            b"field".into(),
            b"value".into(),
        ]
        .into();
        let hset_command: Command = frame.try_into().unwrap();

        let actual = hset_command.execute(backend.clone()).unwrap();
        assert_eq!(actual, 1.into());

        let result = hget_command.execute(backend.clone()).unwrap();
        assert_eq!(result, b"value".into());
    }

    #[test]
    fn test_command_hgetall() {
        let backend = Backend::new();

        let frame: Frame = vec![b"hgetall".into(), b"key".into()].into();
        let hgetall_command: Command = frame.try_into().unwrap();

        let actual = hgetall_command.execute(backend.clone()).unwrap();
        assert_eq!(actual, *NULL);

        let frame: Frame = vec![
            b"hset".into(),
            b"key".into(),
            b"field1".into(),
            b"value1".into(),
        ]
        .into();
        let hset_command: Command = frame.try_into().unwrap();
        hset_command.execute(backend.clone()).unwrap();

        let frame: Frame = vec![
            b"hset".into(),
            b"key".into(),
            b"field2".into(),
            b"value2".into(),
        ]
        .into();
        let hset_command: Command = frame.try_into().unwrap();
        hset_command.execute(backend.clone()).unwrap();

        let result = hgetall_command.execute(backend.clone()).unwrap();

        match result {
            Frame::Array(array) => {
                assert_eq!(array.len(), 4);
                // assert_eq!(array[0], b"field1".into());
                // assert_eq!(array[1], b"value1".into());
                // assert_eq!(array[2], b"field2".into());
                // assert_eq!(array[3], b"value2".into());
            }
            _ => panic!("Expected Array"),
        }
    }
}
