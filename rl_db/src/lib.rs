use rl_utils::Config;
use rusqlite::Connection;
use rusqlite::Result;

pub struct Database<'a> {
    config: &'a Config,
}

impl<'a> Database<'a> {
    pub fn new(config: &'a Config) -> Self {
        Database {
            config
        }
    }

    fn connect(&self) -> Result<Connection> {
        Connection::open(&self.config.database_location)
    }

    pub fn healthcheck(&self) -> Result<bool> {
        let conn = self.connect()?;

        let mut stmt = conn.prepare("SELECT 1")?;

        let result = stmt.query_one([], |result| result.get::<_, f64>(0))?;

        Ok(result.is_finite())
    }
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
