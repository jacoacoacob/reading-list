use crate::{Config, Database};

pub struct Context<'a> {
    pub config: &'a Config,
    pub database: &'a Database<'a>,
}

impl<'a> Context<'a> {
    pub fn new(database: &'a Database<'a>, config: &'a Config) -> Context<'a> {
        Context { database, config }
    }
}
