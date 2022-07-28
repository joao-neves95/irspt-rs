use anyhow::Result;
use irspt_core::traits::DbWrapper;
use sled::Db;

pub struct SledDb {
    pub db: Option<Db>,
}

impl SledDb {
    pub fn new() -> Self {
        SledDb { db: None }
    }
}

impl<'a> DbWrapper<'a> for SledDb {
    fn open(&mut self) -> Result<()> {
        if self.db.is_some() {
            return Ok(());
        }

        // TODO: Un-hardcode the db name.
        self.db = Some(sled::open("data/sled_db")?);
        Ok(())
    }
}
