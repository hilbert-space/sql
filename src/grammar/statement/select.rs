use Result;
use grammar::clause::{OrderBy, Where};
use grammar::{Buffer, Clause, Condition, Expression, Statement};

/// A `SELECT` statement.
#[derive(Debug, Default)]
pub struct Select {
    table: Vec<String>,
    columns: Option<Vec<String>>,
    so_that: Option<Where>,
    order_by: Option<OrderBy>,
    limit: Option<usize>,
}

impl Select {
    /// Create a `SELECT` statement.
    #[inline]
    pub fn new<T: ToString>(table: T) -> Self {
        Select::default().table(table)
    }

    /// Set the table.
    pub fn table<T: ToString>(mut self, name: T) -> Self {
        self.table.push(name.to_string());
        self
    }

    /// Add a column.
    pub fn column<T: ToString>(mut self, name: T) -> Self {
        push!(self.columns, name.to_string());
        self
    }

    /// Add multiple columns.
    pub fn columns<T: ToString>(mut self, names: &[T]) -> Self {
        for name in names {
            push!(self.columns, name.to_string());
        }
        self
    }

    /// Add a condition.
    pub fn so_that<T>(mut self, condition: T) -> Self where T: Condition + 'static {
        self.so_that = Some(match self.so_that.take() {
            Some(so_that) => so_that.and(condition),
            _ => Where::default().and(condition),
        });
        self
    }

    /// Add an order.
    pub fn order_by<T>(mut self, expression: T) -> Self where T: Expression + 'static {
        self.order_by = Some(match self.order_by.take() {
            Some(order_by) => order_by.append(expression),
            _ => OrderBy::default().append(expression),
        });
        self
    }

    /// Set the limit.
    pub fn limit(mut self, count: usize) -> Self {
        self.limit = Some(count);
        self
    }
}

impl Statement for Select {
    fn compile(&self) -> Result<String> {
        let mut buffer = Buffer::new();
        buffer.push("SELECT");
        if let &Some(ref columns) = &self.columns {
            buffer.push({
                let mut buffer = Buffer::new();
                for column in columns {
                    buffer.push(format!("`{}`", column));
                }
                buffer.join(", ")
            });
        } else {
            buffer.push("*");
        }
        buffer.push("FROM");
        let mut do_comma = false;
        for table in &self.table{
            if do_comma{ buffer.push(",");} else{do_comma=true;}
            buffer.push(format!("`{}`", table));
        }
        if let &Some(ref clause) = &self.so_that {
            buffer.push(try!(clause.compile()));
        }
        if let Some(ref clause) = self.order_by {
            buffer.push(try!(clause.compile()));
        }
        if let Some(count) = self.limit {
            buffer.push(format!("LIMIT {}", count));
        }
        Ok(buffer.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn all() {
        let statement = select_from("foo");
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo`");
    }

    #[test]
    fn columns() {
        let statement = select_from("foo").columns(&["bar", "baz"]);
        assert_eq!(statement.compile().unwrap(), "SELECT `bar`, `baz` FROM `foo`");
    }

    #[test]
    fn like() {
        let statement = select_from("foo").so_that(column("bar").like("%baz%"));
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo` WHERE `bar` LIKE '%baz%'");
    }

    #[test]
    fn order() {
        let statement = select_from("foo").order_by("bar").order_by(column("baz").descend());
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo` ORDER BY bar, `baz` DESC");
    }

    #[test]
    fn limit() {
        let statement = select_from("foo").limit(10);
        assert_eq!(statement.compile().unwrap(), "SELECT * FROM `foo` LIMIT 10");
    }
}
