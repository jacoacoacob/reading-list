use rusqlite::Connection;
use rusqlite::Error;
use rusqlite::Params;
use rusqlite::Result;
use rusqlite::Row;
use rusqlite::Statement;
use rusqlite::Transaction;
use rusqlite::fallible_iterator::FallibleIterator;

use crate::Bookmark;
use crate::Config;

impl TryFrom<&Row<'_>> for Bookmark {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> std::result::Result<Self, Self::Error> {
        let tags: String = row.get(6)?;
        let tags: Vec<String> = tags.split(',').map(|x| x.to_string()).collect();

        Ok(Bookmark {
            tags,
            url: row.get(0)?,
            name: row.get(1)?,
            created: row.get(3)?,
            updated: row.get(4)?,
            visited: row.get(5)?,
        })
    }
}

pub struct Database<'c> {
    config: &'c Config,
}

struct QueryExecutor<'c> {
    database: &'c Database<'c>,
    tx: Option<&'c Transaction<'c>>,
    conn: Option<Connection>,
}

impl<'c> QueryExecutor<'c> {
    fn new(
        database: &'c Database<'c>,
        tx: Option<&'c Transaction>
    ) -> QueryExecutor<'c> {
        QueryExecutor {
            database,
            tx,
            conn: match tx {
                Some(_) => None,
                None => Some(database.connect().expect("create database connection")),
            }
        }
    }

    fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        if let Some(tx) = self.tx {
            return tx.execute(sql, params);
        }

        if let Some(conn) = &self.conn {
            return conn.execute(sql, params);
        }

        // TODO: improve error handling here
        Err(Error::InvalidQuery)
    }

    fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        if let Some(tx) = self.tx {
            return tx.prepare(sql);
        }

        if let Some(conn) = &self.conn {
            return conn.prepare(sql);
        }

        // TODO: improve error handling here
        Err(Error::InvalidQuery)
    }
}

impl<'c> Database<'c> {
    pub fn new(config: &'c Config) -> Self {
        Database { config }
    }

    pub fn connect(&self) -> Result<Connection> {
        let conn = Connection::open(&self.config.database_location)?;

        self.create_tables_if_not_exists(&conn)?;

        Ok(conn)
    }

    fn create_tables_if_not_exists(&self, conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "
            BEGIN;

            CREATE TABLE IF NOT EXISTS bookmark (
                url TEXT PRIMARY KEY,
                name TEXT,
                description TEXT,
                created TEXT DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
                updated TEXT DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
                visited TEXT DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
            );

            CREATE TABLE IF NOT EXISTS tag (
                name TEXT PRIMARY KEY
            );

            CREATE TABLE IF NOT EXISTS bookmark_tag (
                bookmark_url TEXT,
                tag_name TEXT,
                FOREIGN KEY (bookmark_url) REFERENCES bookmark (url),
                FOREIGN KEY (tag_name) REFERENCES tag (name),
                PRIMARY KEY (bookmark_url, tag_name)
            );

            COMMIT;
        ",
        )?;

        Ok(())
    }

    pub fn healthcheck(&self) -> Result<()> {
        let conn = self.connect()?;

        let mut stmt = conn.prepare("SELECT datetime('now')")?;

        let result = stmt.query_one([], |result| result.get::<_, String>(0))?;

        if cfg!(feature = "dev") {
            println!("database online at: {result}");
        }

        Ok(())
    }

    pub fn add_bookmark<'b>(
        &self,
        bookmark: &'b Bookmark,
        tx: Option<&Transaction>,
    ) -> Result<&'b Bookmark> {
        let executor = QueryExecutor::new(&self, tx);

        executor.execute(
            "INSERT INTO bookmark (url, name, created, updated, visited)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &bookmark.url,
                &bookmark.name,
                &bookmark.created,
                &bookmark.updated,
                &bookmark.visited,
            ),
        )?;

        for tag in &bookmark.tags {
            executor.execute(
                "INSERT INTO tag (name) VALUES (?1) ON CONFLICT (name) DO NOTHING",
                (tag,),
            )?;

            executor.execute(
                "INSERT INTO bookmark_tag (tag_name, bookmark_url) VALUES ($1, $2)",
                (tag, &bookmark.url),
            )?;
        }

        Ok(bookmark)
    }

    pub fn list_bookmarks(&self, tx: Option<&Transaction>) -> Result<Vec<Bookmark>> {
        let executor = QueryExecutor::new(&self, tx);

        let mut stmt = executor.prepare(
            "
            SELECT b.*, json_group_array(bt.tag_name) tags
            FROM bookmark           b
            LEFT JOIN bookmark_tag  bt
                ON b.url = bt.bookmark_url
            GROUP BY b.url, b.name
        ",
        )?;

        let bookmarks = stmt
            .query([])?
            .map(|row| Bookmark::try_from(row))
            .collect()?;

        Ok(bookmarks)
    }

    pub fn get_bookmark_by_url(&self, url: &str, tx: Option<&Transaction>) -> Result<Bookmark> {
        let executor = QueryExecutor::new(&self, tx);

        let mut stmt = executor.prepare(
            "
            SELECT b.*, json_group_array(bt.tag_name) tags
            FROM bookmark           b
            LEFT JOIN bookmark_tag  bt
                ON b.url = bt.bookmark_url
            WHERE b.url = ?1
            GROUP BY b.url, b.name
        ",
        )?;

        let bookmark = stmt.query_one([url], |row| Bookmark::try_from(row))?;

        Ok(bookmark)
    }

    pub fn delete_bookmark(&self, bookmark: &Bookmark, tx: Option<&Transaction>) -> Result<()> {
        let executor = QueryExecutor::new(&self, tx);

        for tag in &bookmark.tags {
            executor.execute(
                "DELETE FROM bookmark_tag WHERE bookmark_url = ?1 AND tag_name = ?2",
                (&bookmark.url, tag),
            )?;
            executor.execute("DELETE FROM tag WHERE name = ?1", (tag,))?;
        }

        executor.execute("DELETE FROM bookmark WHERE url = ?1", (&bookmark.url,))?;

        Ok(())
    }

    pub fn update_bookmark<'b>(
        &self,
        old: &'b Bookmark,
        new: &'b Bookmark,
        tx: Option<&Transaction>,
    ) -> Result<&'b Bookmark> {
        self.delete_bookmark(old, tx)?;
        self.add_bookmark(new, tx)?;

        Ok(new)
    }
}
