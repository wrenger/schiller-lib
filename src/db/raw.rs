/// Additional database functions
pub trait DatabaseExt {
    fn fetch(&self, statement: &str) -> Result<Vec<Vec<String>>, sqlite::Error>;
}

impl DatabaseExt for sqlite::Connection {
    /// Helper Function that collects the sql result.
    fn fetch(&self, statement: &str) -> Result<Vec<Vec<String>>, sqlite::Error> {
        let mut result = vec![];

        self.iterate(statement, |pairs| {
            result.push(
                pairs
                    .iter()
                    .map(|&(_, value)| value.unwrap_or_default().into())
                    .collect(),
            );
            true
        })?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection() {
        sqlite::Connection::open(":memory:").unwrap();
    }

    #[test]
    fn fetch() {
        let db = sqlite::Connection::open(":memory:").unwrap();
        db.execute("create table abc (a, b, c)").unwrap();
        db.execute("insert into abc values ('a', 'b', 'c')")
            .unwrap();

        let result = db.fetch("select * from abc").unwrap();
        assert_eq!(
            result,
            vec![vec![
                String::from("a"),
                String::from("b"),
                String::from("c")
            ]]
        );
    }
}
