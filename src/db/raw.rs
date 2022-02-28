use super::FromRow;

/// Additional database functions
pub trait DatabaseExt {
    fn fetch(&self, statement: &str) -> Result<Vec<Vec<String>>, rusqlite::Error>;
}

impl DatabaseExt for rusqlite::Connection {
    /// Helper Function that collects the sql result.
    fn fetch(&self, statement: &str) -> Result<Vec<Vec<String>>, rusqlite::Error> {
        let mut stmt = self.prepare(statement)?;
        let mut rows = stmt.query([])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(Vec::<String>::from_row(row)?)
        }
        Ok(result)
    }
}

impl FromRow for Vec<String> {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        (0..row.as_ref().column_count())
            .into_iter()
            .map(|i| row.get(i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection() {
        rusqlite::Connection::open_in_memory().unwrap();
    }

    #[test]
    fn fetch() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        db.execute("create table abc (a, b, c)", []).unwrap();
        db.execute("insert into abc values ('a', 'b', 'c')", [])
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
        let db = rusqlite::Connection::open_in_memory().unwrap();
        db.execute_batch(
            "begin; \
            create table abc (a, b, c); \
            insert into abc values ('d', 'e', 'f'); \
            insert into abc values ('a', 'b', 'c'); \
            commit;",
        )
        .unwrap();

        let result = db.fetch("select * from abc order by a").unwrap();
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
        let db = rusqlite::Connection::open_in_memory().unwrap();
        db.execute("create table abc (a, b, c)", []).unwrap();

        db.execute("insert into abc values (?, ?, ?)", ["1", "2", "3"])
            .unwrap();

        // Explicit binding ids
        db.execute("insert into abc values (?3, ?2, ?1)", ["4", "5", "6"])
            .unwrap();

        assert_eq!(
            vec![
                vec![String::from("1"), String::from("2"), String::from("3")],
                vec![String::from("6"), String::from("5"), String::from("4")]
            ],
            db.fetch("select * from abc").unwrap()
        );

        let num: String = db
            .query_row("select a from abc where a=\"1\"", [], |row| row.get(0))
            .unwrap();
        assert_eq!(num, "1");
    }

    #[test]
    fn transaction() {
        let mut db = rusqlite::Connection::open_in_memory().unwrap();
        db.execute("create table abc (a not null, b, c)", [])
            .unwrap();

        {
            let transaction = db.transaction().unwrap();

            transaction
                .execute("insert into abc values (?, ?, ?)", ["4", "5", "6"])
                .unwrap();
            // no commit -> rollback
        };
        assert!(db.fetch("select * from abc").unwrap().is_empty());

        {
            let transaction = db.transaction().unwrap();

            transaction
                .execute("insert into abc values (?, ?, ?)", ["5", "6"])
                .expect_err("Null violation!");
            // no commit -> rollback
        };
        assert!(db.fetch("select * from abc").unwrap().is_empty());

        {
            let transaction = db.transaction().unwrap();

            transaction
                .execute("insert into abc values (?, ?, ?)", ["1", "2", "3"])
                .unwrap();
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
