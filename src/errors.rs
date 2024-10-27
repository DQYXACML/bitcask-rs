use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("failed to read from data file")]
    FailedReadFromDataFile,

    #[error("failed to write to data file")]
    FailedWriteToDataFile,

    #[error("failed to sync to data file")]
    FailedSyncDataFile,

    #[error("failed to Open data file")]
    FailedOpenDataFile,
}

pub type Result<T> = result::Result<T, Errors>;
