use crate::{db::Db, errors::DirtyError};

pub struct Interface {
    database: Db
}

impl Interface {

    pub fn new() -> Self {
        let db = Db::new("".into());
        Self {
            database: db
        }
    }

    pub async fn withdraw(&self, _userid: String, _addr: String, _value: f64) {

    }

    pub async fn deposit(&self, _userid: String) {

    }

    pub async fn send(&self, _userid: String, _value: f64) {

    }

    pub async fn balance(&self, userid: String) -> Result<String, DirtyError> {
        let user = self.database.find_user(userid).await?;

        return Ok(user.amount);
    }
}