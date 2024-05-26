use crate::storage::memory::MemoryStore;
use crate::{
    resource::{Generic, ResourceGroup, ResourceMetadata},
    storage::ResourceStore,
};
use resource::{Namespace, Project, Resource};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod resource;
mod storage;

fn main() {
    run().expect("there was an error");
}

fn run() -> anyhow::Result<()> {
    let mut store = MemoryStore::default();
    let project_definition = Project::resource_definition();
    store.write(&project_definition)?;

    let default = Project::new("default");
    let ns = Namespace::new(&default.resource_ref(), "default");

    store.write(&default)?;
    store.write(&ns)?;

    #[derive(Deserialize, Serialize, Clone)]
    struct Repo {
        pub url: String,
    }

    #[derive(Deserialize, Serialize, Clone)]
    struct PipelineProvider {
        pub url: String,
    }

    #[derive(Deserialize, Serialize, Clone)]
    struct PipelineConfig {
        pub url: String,
    }

    #[derive(Deserialize, Serialize, Clone)]
    struct PipelineRun {
        pub url: String,
        pub pass: bool,
    }

    let pipeline_provider = Generic {
        group: ResourceGroup {
            api_version: "josh/v1".to_string(),
            kind: "PipelineProvider".to_string(),
        },
        metadata: ResourceMetadata {
            name: "github-actions".to_string(),
            owner_ref: Some(ns.resource_ref()),
            ..Default::default()
        },
        spec: PipelineProvider {
            url: "https://github.com".to_string(),
        },
    };

    store.write(&pipeline_provider)?;

    let pipeline_config = Generic {
        group: ResourceGroup {
            api_version: "josh/v1".to_string(),
            kind: "PipelineConfig".to_string(),
        },
        metadata: ResourceMetadata {
            name: "build-image".to_string(),
            owner_ref: Some(pipeline_provider.resource_ref()),
            ..Default::default()
        },
        spec: PipelineProvider {
            // doesn't actually exist....
            url: "https://github.com/kasuboski/controlplane/blob/ad5b20cb7c99fa1b4c353638aaa32bb773b42d74/.github/workflows/build.yaml".to_string(),
        },
    };

    store.write(&pipeline_config)?;

    let pipeline_run = Generic {
        group: ResourceGroup {
            api_version: "josh/v1".to_string(),
            kind: "PipelineRun".to_string(),
        },
        metadata: ResourceMetadata {
            name: "run-1".to_string(),
            owner_ref: Some(pipeline_config.resource_ref()),
            labels: BTreeMap::from([("joshcorp.co/repo".to_string(), "controlplane".to_string())]),
            ..Default::default()
        },
        spec: PipelineRun {
            url: "https://github.com/controlplane/actions/runs/1".to_string(),
            pass: true,
        },
    };
    store.write(&pipeline_run)?;

    let repo = Generic {
        group: ResourceGroup {
            api_version: "josh/v1".to_string(),
            kind: "Repo".to_string(),
        },
        metadata: ResourceMetadata {
            name: "controlplane".to_string(),
            owner_ref: Some(ns.resource_ref()),
            ..Default::default()
        },
        spec: Repo {
            url: "https://github.com/kasuboski/controlplane".to_string(),
        },
    };

    store.write(&repo)?;
    let read: Generic<Repo> = store.read(&repo.resource_ref())?;
    println!("read: {}", serde_json::to_string_pretty(&read)?);

    let read: Generic<PipelineRun> = store.read(&pipeline_run.resource_ref())?;
    println!("read: {}", serde_json::to_string_pretty(&read)?);
    Ok(())
}
