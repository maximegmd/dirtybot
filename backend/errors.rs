use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum DirtyError {
	Blockchain(web3::Error),
	InvalidAddress,
	InvalidPassphrase,
}

impl From<web3::Error> for DirtyError {
	fn from(e: web3::Error) -> Self {
		DirtyError::Blockchain(e)
	}
}

impl Error for DirtyError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			DirtyError::Blockchain(ref e) => Some(e),
			_ => None,
		}
	}
}

impl Display for DirtyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			DirtyError::Blockchain(ref e) => write!(f, "Blockchain error: {}", e),
			DirtyError::InvalidAddress => write!(f, "Invalid address"),
			DirtyError::InvalidPassphrase => write!(f, "Invalid passphrase"),
		}
	}
}
