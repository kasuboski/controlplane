pub mod memory;

use crate::resource::{Ref, Resource};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

type Result<T> = std::result::Result<T, StorageError>;

pub trait ResourceStore {
    fn write<R: Serialize + Resource>(&mut self, res: &R) -> Result<()>;
    fn read<'s: 'r, 'r, R: DeserializeOwned>(&'s self, key: &Ref) -> Result<R>;
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("resource not found")]
    ResourceNotFound,
    #[error("couldn't serialize")]
    SerializationError(#[from] serde_json::Error),
    #[error("unknown data store error")]
    Unknown,
}
