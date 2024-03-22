use crate::{resource::{Generic, ResourceGroup, ResourceMetadata}, storage::ResourceStore};
use resource::{Namespace, Project, Resource};
use serde::{Deserialize, Serialize};
use storage::MemoryStore;

mod resource;
mod storage;

fn main() {
    run().expect("there was an error");
}

fn run() -> anyhow::Result<()> {
    let mut store = MemoryStore::default();
    let default = Project::new("default");
    let ns = Namespace::new(&default.resource_ref(), "default");

    store.write(&default)?;
    store.write(&ns)?;

    #[derive(Deserialize, Serialize, Clone)]
    struct MyResource {
        pub hello: String,
    }

    let mine = Generic {
        group: ResourceGroup {
            api_version: "josh/v1".to_string(),
            kind: "MyResource".to_string(),
        },
        metadata: ResourceMetadata {
            name: "mine".to_string(),
            owner_ref: Some(ns.resource_ref()),
            ..Default::default()
        },
        spec: MyResource {
            hello: "world".to_string(),
        },
    };

    store.write(&mine)?;
    let read: Generic<MyResource> = store.read(&mine.resource_ref())?;
    println!("read: {}", serde_json::to_string_pretty(&read)?);
    Ok(())
}
