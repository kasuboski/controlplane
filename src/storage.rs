use std::collections::HashMap;
use std::sync::Mutex;

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::resource::{Ref, Resource};

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

#[derive(Default)]
pub struct MemoryStore(Mutex<HashMap<Ref, String>>);
impl ResourceStore for MemoryStore {
    fn write<R: Serialize + Resource>(&mut self, res: &R) -> Result<()> {
        if let Ok(mut l) = self.0.try_lock() {
            let value = serde_json::to_string(res)?;
            let key: Ref = res.resource_ref();
            l.insert(key, value);
            return Ok(());
        }
        Err(StorageError::Unknown)
    }

    fn read<'s: 'r, 'r, R: DeserializeOwned>(&'s self, key: &Ref) -> Result<R> {
        if let Ok(l) = self.0.try_lock() {
            let value = l.get(key).ok_or(StorageError::ResourceNotFound)?;
            let value = value.clone();
            let out: R = serde_json::from_str(&value)?;
            return Ok(out);
        }

        Err(StorageError::Unknown)
    }
}

#[cfg(test)]
mod test {
    use crate::resource::{Namespace, Project, Resource};

    use super::*;

    #[test]
    fn store_read_a_write() {
        let mut store = MemoryStore::default();
        let project = Project::new("test");
        store.write(&project).expect("couldn't write project");
        let out: Project = store
            .read(&project.resource_ref())
            .expect("couldn't read project");

        assert_eq!(out.metadata.name, "test");
    }

    #[test]
    fn store_not_found() {
        let store = MemoryStore::default();
        let out: Result<Namespace> = store.read(&Ref {
            api_version: "nonsense".to_string(),
            kind: "test".to_string(),
            name: "test".to_string(),
        });

        let Err(StorageError::ResourceNotFound) = out else {
            panic!("didn't get the not found error");
        };
    }
}
