use crate::storage::ResourceStore;
use resource::{Namespace, Project, Resource};
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

    let read: Namespace = store.read(&ns.resource_ref())?;
    println!("read: {}", serde_json::to_string_pretty(&read)?);
    Ok(())
}
