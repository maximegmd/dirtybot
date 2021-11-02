use diesel::Queryable;

#[derive(Queryable, Clone)]
pub struct User {
    pub discord_id: String,
    pub deposit_address: String,
    pub amount: String
}