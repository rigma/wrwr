use serde::{de, ser};
use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An error was emitted from the reader/writer used during serialization/deserialization process!")]
    Io(#[from] io::Error),
    #[error("A sequence, tuple or a map has tried to serialize/deserialize more element that anticipated!")]
    SizeLimit(usize),
    #[error("A custom error was emitted from Serde. Message: {0}")]
    Custom(String),
}

impl de::Error for Error {
    fn custom<D>(msg: D) -> Self
    where
        D: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<D>(msg: D) -> Self
    where
        D: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
