use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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

impl Resource for Project {
    fn resource_ref(&self) -> Ref {
        Ref {
            api_version: Project::API_VERSION.to_string(),
            kind: Project::KIND.to_string(),
            name: self.metadata.name.clone(),
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

impl Resource for Namespace {
    fn resource_ref(&self) -> Ref {
        Ref {
            api_version: Namespace::API_VERSION.to_string(),
            kind: Namespace::KIND.to_string(),
            name: self.metadata.name.clone(),
        }
    }
}

/// Generic represents a user given resource
#[derive(Deserialize, Serialize, Clone)]
pub struct Generic<T> {
    #[serde(flatten)]
    pub group: ResourceGroup,
    pub metadata: ResourceMetadata,
    pub spec: T,
}

impl<T> Resource for Generic<T> {
    fn resource_ref(&self) -> Ref {
        Ref {
            api_version: self.group.api_version.clone(),
            kind: self.group.kind.clone(),
            name: self.metadata.name.clone(),
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

pub trait Resource {
    fn resource_ref(&self) -> Ref;
}
