use ion_binary_rs::IonParserError;
use rusoto_core::{request::TlsError, RusotoError};
use rusoto_qldb_session::SendCommandError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QLDBError {
    #[error("The QLDB command returned an error")]
    SendCommandError(#[from] RusotoError<SendCommandError>),
    #[error("We requested a session but QLDB returned nothing")]
    QLDBReturnedEmptySession,
    #[error("We requested a transaction id but QLDB returned nothing")]
    QLDBReturnedEmptyTransaction,
    // TODO: IonParserError seems to not be compatible
    // with "#[from]". Make it compatible
    #[error("We requested a transaction id but QLDB returned nothing")]
    IonParserError(IonParserError),
    #[error("Error when creating the HttpClient")]
    TlsError(#[from] TlsError),
    #[error("Transaction has been already commit or rollback")]
    TransactionCompleted,
    #[error("We weren't able to send the result value to ourselves. This is a bug.")]
    InternalChannelSendError,
    #[error("We weren't able to receive the result value from ourselves. This is a bug.")]
    InternalChannelRecvError(async_channel::RecvError),
}

pub type QLDBResult<T> = Result<T, QLDBError>;
