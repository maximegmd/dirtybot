use tokio::task::JoinError;

#[derive(Debug)]
pub enum DirtyError {
    Blockchain(web3::Error),
    InvalidAddress,
    InvalidPassphrase,
    DatabaseFailure(diesel::result::Error),
    NotFound,
    Abort
}

impl From<web3::Error> for DirtyError {
    fn from(e: web3::Error) -> Self {
        DirtyError::Blockchain(e)
    }
}

impl From<diesel::result::Error> for DirtyError {
    fn from(e: diesel::result::Error) -> Self {
        DirtyError::DatabaseFailure(e)
    }
}


impl From<JoinError> for DirtyError {
    fn from(_e: JoinError) -> Self {
        DirtyError::Abort
    }
}