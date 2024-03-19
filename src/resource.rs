use std::collections::BTreeMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Resources {
    Project(Project),
    Namespace(Namespace),
    Generic(Ref),
}

impl Resource for Resources {
    fn resource_ref(&self) -> Ref {
        match self {
            Resources::Project(p) => Ref {
                api_version: Project::API_VERSION.to_string(),
                kind: Project::KIND.to_string(),
                name: p.metadata.name.clone(),
            },
            Resources::Namespace(ns) => Ref {
                api_version: Namespace::API_VERSION.to_string(),
                kind: Namespace::KIND.to_string(),
                name: ns.metadata.name.clone(),
            },
            Resources::Generic(r) => r.clone(),
        }
    }
}

/// Project represents the broadest tenant. It contains all other resources.
#[derive(Deserialize, Serialize, Clone)]
pub struct Project {
    #[serde(flatten)]
    pub group: ResourceGroup,
    pub metadata: ResourceMetadata,
}

impl Project {
    const API_VERSION: &'static str = "core/v1";
    const KIND: &'static str = "project";

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            group: ResourceGroup {
                api_version: Project::API_VERSION.to_string(),
                kind: Project::KIND.to_string(),
            },
            metadata: ResourceMetadata {
                name: name.into(),
                ..Default::default()
            },
        }
    }
}

/// Namespace represents a slice of resources.
#[derive(Deserialize, Serialize, Clone)]
pub struct Namespace {
    #[serde(flatten)]
    pub group: ResourceGroup,
    pub metadata: ResourceMetadata,
}

impl Namespace {
    const API_VERSION: &'static str = "core/v1";
    const KIND: &'static str = "namespace";

    pub fn new(project: &Ref, name: impl Into<String>) -> Self {
        Self {
            group: ResourceGroup {
                api_version: Namespace::API_VERSION.to_string(),
                kind: Namespace::KIND.to_string(),
            },
            metadata: ResourceMetadata {
                name: name.into(),
                owner_ref: Some(project.clone()),
                ..Default::default()
            },
        }
    }
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ResourceMetadata {
    pub name: String,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    pub owner_ref: Option<Ref>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceGroup {
    pub api_version: String,
    pub kind: String,
}

/// A Ref refers to a resource
#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct Ref {
    pub api_version: String,
    pub kind: String,
    pub name: String,
}

pub trait Resource: DeserializeOwned + Serialize + Clone {
    fn resource_ref(&self) -> Ref;
}
