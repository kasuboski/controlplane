use crate::storage::ResourceStore;
use resource::{Namespace, Project, Resource, Resources};
use storage::MemoryStore;

mod resource;
mod storage;

fn main() {
    run().expect("there was an error");
}

fn run() -> anyhow::Result<()> {
    let mut store = MemoryStore::default();
    let default = Resources::Project(Project::new("default"));
    let ns = Resources::Namespace(Namespace::new(&default.resource_ref(), "default"));

    store.write(&default)?;
    store.write(&ns)?;

    let read: Resources = store.read(&ns.resource_ref())?;
    println!("read: {}", serde_json::to_string_pretty(&read)?);
    Ok(())
}
