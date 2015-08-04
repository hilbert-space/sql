//! A constructor of SQL statements.

use std::default::Default;
use std::{error, fmt, result};

struct Buffer(Vec<String>);

/// An error.
pub struct Error(String);

/// A result.
pub type Result<T> = result::Result<T, Error>;

/// A data type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    /// The binary type.
    Binary,
    /// The floating-point type.
    Float,
    /// The integer type.
    Integer,
    /// The string type.
    String,
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.0
    }
}

impl Buffer {
    fn new() -> Buffer {
        Buffer(vec![])
    }

    fn push<T: ToString>(&mut self, chunk: T) -> &mut Self {
        self.0.push(chunk.to_string());
        self
    }

    fn join(self, delimiter: &str) -> String {
        let mut result = String::new();
        for (i, chunk) in self.0.iter().enumerate() {
            if i > 0 {
                result.push_str(delimiter)
            }
            result.push_str(chunk);
        }
        result
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

macro_rules! raise(
    ($message:expr) => (
        return Err(::Error($message.to_string()));
    );
);

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => raise!(error),
        }
    );
);

macro_rules! some(
    ($from:ident, $what:ident) => (
        match $from.$what {
            Some(ref value) => value,
            _ => raise!(concat!("expected `", stringify!($what), "` to be set")),
        }
    );
);

pub mod definition;
pub mod expression;
pub mod operation;
pub mod statement;

pub mod prelude;

/// Create a column definition.
#[inline]
pub fn column() -> definition::Column {
    Default::default()
}

/// Create a `CREATE TABLE` statement.
#[inline]
pub fn create_table() -> statement::CreateTable {
    Default::default()
}

/// Create an `INSERT INTO` statement.
#[inline]
pub fn insert_into() -> statement::InsertInto {
    Default::default()
}

/// Create a `SELECT` statement.
#[inline]
pub fn select() -> statement::Select {
    Default::default()
}
