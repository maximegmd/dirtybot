use diesel::Queryable;

#[derive(Queryable, Clone, Debug)]
pub struct User {
	pub id: String,
	pub deposit_address: String,
	pub amount: String,
}
