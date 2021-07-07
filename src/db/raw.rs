use std::collections::HashMap;

/// Additional database functions
pub trait DatabaseExt {
    fn fetch(&self, statement: &str) -> Result<Vec<Vec<String>>, sqlite::Error>;
    fn transaction(&self) -> Result<Transaction, sqlite::Error>;
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

    fn transaction(&self) -> Result<Transaction, sqlite::Error> {
        self.execute("begin")?;
        Ok(Transaction { db: self })
    }
}

pub struct Transaction<'a> {
    db: &'a sqlite::Connection,
}

impl<'a> Transaction<'a> {
    pub fn commit(self) -> Result<(), sqlite::Error> {
        self.db.execute("commit")?;
        std::mem::forget(self);
        Ok(())
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        self.db.execute("rollback").ok();
    }
}

pub trait StatementExt {
    fn columns(&self) -> HashMap<String, usize>;
}

impl<'a> StatementExt for sqlite::Statement<'a> {
    fn columns(&self) -> HashMap<String, usize> {
        self.column_names()
            .into_iter()
            .enumerate()
            .map(|(i, col)| (col.to_string(), i))
            .collect()
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
            vec![vec![
                String::from("a"),
                String::from("b"),
                String::from("c")
            ]],
            result
        );
    }

    #[test]
    fn multiple() {
        let db = sqlite::Connection::open(":memory:").unwrap();
        db.execute(
            "begin; \
            create table abc (a, b, c); \
            insert into abc values ('d', 'e', 'f'); \
            commit;",
        )
        .unwrap();

        let result = db
            .fetch(
                "insert into abc values ('a', 'b', 'c'); \
                select * from abc order by a",
            )
            .unwrap();
        assert_eq!(
            vec![
                vec![String::from("a"), String::from("b"), String::from("c")],
                vec![String::from("d"), String::from("e"), String::from("f")]
            ],
            result
        );
    }

    #[test]
    fn prepare() {
        let db = sqlite::Connection::open(":memory:").unwrap();
        db.execute("create table abc (a, b, c)").unwrap();

        let mut stmt = db.prepare("insert into abc values (?, ?, ?)").unwrap();
        stmt.bind(1, "1").unwrap();
        stmt.bind(2, "2").unwrap();
        stmt.bind(3, "3").unwrap();
        assert_eq!(stmt.next().unwrap(), sqlite::State::Done);

        // Explicit binding ids
        let mut stmt = db.prepare("insert into abc values (?3, ?2, ?1)").unwrap();
        stmt.bind(1, "4").unwrap();
        stmt.bind(2, "5").unwrap();
        stmt.bind(3, "6").unwrap();
        assert_eq!(stmt.next().unwrap(), sqlite::State::Done);

        assert_eq!(
            vec![
                vec![String::from("1"), String::from("2"), String::from("3")],
                vec![String::from("6"), String::from("5"), String::from("4")]
            ],
            db.fetch("select * from abc").unwrap()
        );

        let mut stmt = db.prepare("select a from abc where a='1'").unwrap();
        assert_eq!(stmt.next().unwrap(), sqlite::State::Row);
        assert_eq!(stmt.read::<i64>(0).unwrap(), 1);
        assert_eq!(stmt.next().unwrap(), sqlite::State::Done);
    }

    #[test]
    fn transaction() {
        let db = sqlite::Connection::open(":memory:").unwrap();
        db.execute("create table abc (a not null, b, c)").unwrap();

        {
            let _transaction = db.transaction().unwrap();

            let mut stmt = db.prepare("insert into abc values (?, ?, ?)").unwrap();
            stmt.bind(1, "4").unwrap();
            stmt.bind(2, "5").unwrap();
            stmt.bind(3, "6").unwrap();
            assert_eq!(stmt.next().unwrap(), sqlite::State::Done);

            // no commit -> rollback
        };
        assert!(db.fetch("select * from abc").unwrap().is_empty());

        {
            let _transaction = db.transaction().unwrap();

            let mut stmt = db.prepare("insert into abc values (?, ?, ?)").unwrap();
            stmt.bind(2, "5").unwrap();
            stmt.bind(3, "6").unwrap();
            stmt.next().expect_err("Null violation!");
            // no commit -> rollback
        };
        assert!(db.fetch("select * from abc").unwrap().is_empty());

        {
            let transaction = db.transaction().unwrap();

            let mut stmt = db.prepare("insert into abc values (?, ?, ?)").unwrap();
            stmt.bind(1, "1").unwrap();
            stmt.bind(2, "2").unwrap();
            stmt.bind(3, "3").unwrap();
            assert_eq!(stmt.next().unwrap(), sqlite::State::Done);

            println!("finish -> commit");
            transaction.commit().unwrap();
        };
        assert_eq!(
            vec![vec![
                String::from("1"),
                String::from("2"),
                String::from("3")
            ],],
            db.fetch("select * from abc").unwrap()
        );
    }
}
