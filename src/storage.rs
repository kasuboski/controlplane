use std::collections::HashMap;
use std::sync::Mutex;

use thiserror::Error;

use crate::resource::{Ref, Resource};

type Result<T> = std::result::Result<T, StorageError>;

pub trait ResourceStore {
    fn write<R: Resource>(&mut self, res: &R) -> Result<()>;
    fn read<'s: 'r, 'r, R: Resource>(&'s self, key: &Ref) -> Result<R>;
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("resource not found")]
    ResourceNotFound,
    #[error("couldn't serialize")]
    SerializationError(serde_json::Error),
    #[error("unknown data store error")]
    Unknown,
}

impl From<serde_json::Error> for StorageError {
    fn from(e: serde_json::Error) -> StorageError {
        Self::SerializationError(e)
    }
}

#[derive(Default)]
pub struct MemoryStore(Mutex<HashMap<Ref, String>>);
impl ResourceStore for MemoryStore {
    fn write<R: Resource>(&mut self, res: &R) -> Result<()> {
        if let Ok(mut l) = self.0.try_lock() {
            let value = serde_json::to_string(res)?;
            let key: Ref = res.resource_ref();
            l.insert(key, value);
            return Ok(());
        }
        Err(StorageError::Unknown)
    }

    fn read<'s: 'r, 'r, R: Resource>(&'s self, key: &Ref) -> Result<R> {
        if let Ok(l) = self.0.try_lock() {
            let value = l.get(key).ok_or(StorageError::ResourceNotFound)?;
            let value = value.clone();
            let out: R = serde_json::from_str(&value)?;
            return Ok(out.clone());
        }

        Err(StorageError::Unknown)
    }
}

#[cfg(test)]
mod test {
    use crate::resource::{Project, Resource, Resources};

    use super::*;

    #[test]
    fn store_read_a_write() {
        let mut store = MemoryStore::default();
        let project = Resources::Project(Project::new("test"));
        store.write(&project).expect("couldn't write project");
        let out: Resources = store
            .read(&project.resource_ref())
            .expect("couldn't read project");
        let Resources::Project(project) = out else {
            panic!("Didn't get a project back");
        };

        assert_eq!(project.metadata.name, "test")
    }

    #[test]
    fn store_not_found() {
        let store = MemoryStore::default();
        let out: Result<Resources> = store.read(&Ref {
            api_version: "nonsense".to_string(),
            kind: "test".to_string(),
            name: "test".to_string(),
        });

        let Err(StorageError::ResourceNotFound) = out else {
            panic!("didn't get the not found error");
        };
    }
}
