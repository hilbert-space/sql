//! The grammar.

use std::fmt::Debug;

use Result;

struct Buffer(Vec<String>);

/// A clause.
pub trait Clause: Debug {
    /// Compile the clause.
    fn compile(&self) -> Result<String>;
}

/// A condition.
pub trait Condition: Debug {
    /// Compile the condition.
    fn compile(&self) -> Result<String>;
}

/// A definition.
pub trait Definition: Debug {
    /// Compile the definition.
    fn compile(&self) -> Result<String>;
}

/// An expression.
pub trait Expression: Debug {
    /// Compile the expression.
    fn compile(&self) -> Result<String>;
}

/// An operation.
pub trait Operation: Debug {
    /// Compile the operation.
    fn compile(&self) -> Result<String>;
}

/// A statement.
pub trait Statement: Debug {
    /// Compile the statement.
    fn compile(&self) -> Result<String>;
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

macro_rules! string {
    ($($kind:ty),*) => (
        $(
            impl<'l> $kind for &'l str {
                #[inline]
                fn compile(&self) -> Result<String> {
                    Ok(self.to_string())
                }
            }

            impl $kind for String {
                #[inline]
                fn compile(&self) -> Result<String> {
                    Ok(self.clone())
                }
            }
        )*
    );
}

string!(Clause, Condition, Definition, Expression, Operation, Statement);

macro_rules! some(
    ($option:expr, $name:expr) => (
        match $option {
            Some(ref value) => value,
            _ => raise!(concat!("expected “", stringify!($name), "” to be set")),
        }
    );
    ($this:ident.$field:ident) => (
        some!($this.$field, $field)
    );
);


macro_rules! first(
    ($arr:expr, $name:expr) => (
        match $arr.first() {
            Some(ref value) => value,
            _ => raise!(concat!("expected “", stringify!($name), "” to have at least 1 specified")),
        }
    );
    ($this:ident.$field:ident) => (
        first!($this.$field, $field)
    );
);

macro_rules! push(
    ($collection:expr, $value:expr) => (
        match $collection {
            Some(ref mut collection) => {
                collection.push($value);
            },
            _ => {
                let collection = &mut $collection;
                *collection = Some(vec![$value]);
            },
        }
    );
);

pub mod clause;
pub mod definition;
pub mod operation;
pub mod statement;
