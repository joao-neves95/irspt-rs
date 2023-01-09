use anyhow::Result;
use sled::Db;

pub struct SledDb {
    db: Option<Db>,
}

impl SledDb {
    pub fn new() -> Self {
        SledDb { db: None }
    }
}

impl SledDb {
    pub fn db_ref(&self) -> Option<&Db> {
        if self.db.is_none() {
            return None;
        }

        Some(&self.db.as_ref().unwrap())
    }

    pub fn open(&mut self) -> Result<()> {
        if self.db.is_some() {
            return Ok(());
        }

        // TODO: Un-hardcode the db name.
        self.db = Some(sled::open("data/sled_db")?);
        Ok(())
    }
}
