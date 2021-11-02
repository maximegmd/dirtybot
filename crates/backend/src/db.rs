use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use crate::{errors::DirtyError, models::User};
use crate::schema::users::dsl::*;
use diesel::ExpressionMethods;

pub struct Db {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>
}

impl Db {
    pub fn new(uri: String) -> Self {

        let manager = ConnectionManager::<PgConnection>::new(uri);
        let pool = r2d2::Pool::new(manager).unwrap();

        Self {
            pool
        }
    }

    pub async fn find_user(&self, id: String) -> Result<User, DirtyError> {

        let db = self.pool.get().unwrap();

        let user = tokio::task::spawn_blocking(move || -> Result<User, DirtyError> {
            let result = users.filter(discord_id.eq(&id))
                .load::<User>(&*db)?;

            if result.len() < 1 {
                return Err(DirtyError::NotFound);
            }

            let user = result[0].clone();
            
            Ok(user)
        }).await??;

        Ok(user)
    }
}