use crate::schema::users::dsl::*;
use crate::{blockchain::Blockchain, errors::DirtyError};
use diesel::{prelude::*, result::Error as DbError};
use napi::bindgen_prelude::*;
use std::convert::From;
use web3::types::U256;

impl From<DirtyError> for Error {
	fn from(e: DirtyError) -> Error {
		let status = match e {
			DirtyError::Blockchain(_) => Status::GenericFailure,
			_ => Status::InvalidArg,
		};
		Error::new(status, e.to_string())
	}
}

#[napi(object)]
pub struct ContextConfig {
	pub rpc_endpoint: String,
	pub passphrase: String,
	pub database_url: String,
}

#[napi]
pub struct Context {
	db_connection: SqliteConnection,
	blockchain: Blockchain,
}

#[napi]
impl Context {
	#[napi(constructor)]
	pub fn new(config: ContextConfig) -> Result<Self> {
		let db_connection = SqliteConnection::establish(&config.database_url).map_err(|e| match e {
			ConnectionError::InvalidCString(_) | ConnectionError::InvalidConnectionUrl(_) => {
				Error::new(Status::InvalidArg, e.to_string())
			}
			_ => Error::from_reason(e.to_string()),
		})?;

		let blockchain = Blockchain::new(&config.rpc_endpoint, &config.passphrase)?;

		Ok(Self {
			db_connection,
			blockchain,
		})
	}

	pub fn withdraw(&self, _user_id: String, _addr: String, _value: U256) {
		todo!();
	}

	pub fn deposit(&self, _user_id: String) {
		todo!();
	}

	pub fn send(&self, _user_id: String, _value: U256) {
		todo!();
	}

	#[napi]
	pub fn balance(&self, user_id: String) -> Result<String> {
		users
			.find(user_id)
			.select(amount)
			.first::<String>(&self.db_connection)
			.map_err(|e| match e {
				DbError::NotFound => Error::from_reason("User not found".into()),
				_ => Error::from_reason(e.to_string()),
			})
	}
}
