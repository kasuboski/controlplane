use std::collections::BTreeMap;

use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

/// Project represents the broadest tenant. It contains all other resources.
#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct Project {
    #[serde(flatten)]
    pub group: ResourceGroup,
    pub metadata: ResourceMetadata,
}

impl Project {
    const API_VERSION: &'static str = "core/v1";
    const KIND: &'static str = "project";

    pub fn new(name: impl Into<String>) -> Self {
        let mut default = Self::default();
        default.metadata.name = name.into();
        default
    }

    pub fn resource_definition() -> ResourceDefinition {
        let schema = schema_for!(Project);
        let value = serde_json::to_value(schema).expect("schema must be valid");
        ResourceDefinition::new(ResourceDefinitionSpec {
            group: "core".into(),
            names: ResourceNames {
                kind: Project::KIND.into(),
            },
            versions: vec![ResourceVersion {
                name: "v1".into(),
                schema: ResourceSchema::JsonSchema(value),
            }],
        })
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            group: ResourceGroup {
                api_version: Project::API_VERSION.to_string(),
                kind: Project::KIND.to_string(),
            },
            metadata: ResourceMetadata::default(),
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
#[derive(Deserialize, Serialize, Clone, JsonSchema)]
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

#[derive(Deserialize, Serialize, Default, Clone, JsonSchema)]
pub struct ResourceMetadata {
    pub name: String,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub owner_ref: Option<Ref>,
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct ResourceGroup {
    pub api_version: String,
    pub kind: String,
}

/// A Ref refers to a resource
#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Hash, JsonSchema)]
pub struct Ref {
    pub api_version: String,
    pub kind: String,
    pub name: String,
}

pub trait Resource {
    fn resource_ref(&self) -> Ref;
}

/// A ResourceDefinition outlines how a resource is created
#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceDefinition {
    #[serde(flatten)]
    pub group: ResourceGroup,
    pub metadata: ResourceMetadata,
    pub spec: ResourceDefinitionSpec,
}

impl ResourceDefinition {
    const API_VERSION: &'static str = "core/v1";
    const KIND: &'static str = "resourcedefinition";

    fn new(spec: ResourceDefinitionSpec) -> ResourceDefinition {
        let mut metadata = ResourceMetadata::default();
        metadata.name = format!(
            "{}/{}",
            ResourceDefinition::API_VERSION.to_string(),
            ResourceDefinition::KIND.to_string()
        );
        ResourceDefinition {
            group: ResourceGroup {
                api_version: ResourceDefinition::API_VERSION.into(),
                kind: ResourceDefinition::KIND.into(),
            },
            metadata: ResourceMetadata::default(),
            spec,
        }
    }
}

impl Resource for ResourceDefinition {
    fn resource_ref(&self) -> Ref {
        Ref {
            api_version: ResourceDefinition::API_VERSION.to_string(),
            kind: ResourceDefinition::KIND.to_string(),
            name: self.metadata.name.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceDefinitionSpec {
    pub group: String,
    pub names: ResourceNames,
    pub versions: Vec<ResourceVersion>,
}

/// ResourceVersions capture the different schema between versions
#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceVersion {
    pub name: String,
    pub schema: ResourceSchema,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceNames {
    pub kind: String,
}

/// ResourceSchema has the validation options for a resource version
#[derive(Deserialize, Serialize, Clone)]
pub enum ResourceSchema {
    JsonSchema(serde_json::Value),
}

#[cfg(test)]
mod tests {
    use jsonschema::JSONSchema;

    use super::*;

    #[test]
    fn test_project_definition() {
        let project_def = Project::resource_definition();
        let versions = project_def.spec.versions;
        assert_eq!(versions.len(), 1);
        let schema = versions
            .first()
            .expect("couldn't get first version")
            .schema
            .clone();
        let schema = match schema {
            ResourceSchema::JsonSchema(s) => s,
        };

        let compiled = JSONSchema::compile(&schema).expect("not a valid jsonschema");
        let project = Project::new("mine");
        let json = serde_json::to_value(&project).expect("couldn't serialize project");
        let result = compiled.validate(&json);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
                panic!();
            }
        }
    }
}
