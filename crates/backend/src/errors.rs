pub enum DirtyError {
	Blockchain(web3::Error),
}

impl From<web3::Error> for DirtyError {
	fn from(e: web3::Error) -> Self {
		DirtyError::Blockchain(e)
	}
}
