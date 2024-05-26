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
        let mut default = Self::default();
        default.metadata.name = name.into();
        default
    }

    pub fn resource_definition() -> ResourceDefinition {
        ResourceDefinition {
            group: "core".to_string(),
            kind: Project::KIND.to_string(),
            versions: vec![ResourceVersion {
                schema: ResourceSchema::JsonSchema(serde_json::json!(
                    {
                        "$schema": "http://json-schema.org/draft-07/schema#",
                        "title": "Generated schema for Root",
                        "type": "object",
                        "properties": {
                          "api_version": {
                            "type": "string"
                          },
                          "kind": {
                            "type": "string"
                          },
                          "metadata": {
                            "type": "object",
                            "properties": {
                              "name": {
                                "type": "string"
                              },
                              "labels": {
                                "type": "object",
                              },
                              "annotations": {
                                "type": "object",
                              },
                              "owner_ref": {
                                "type": "object",
                                "properties": {
                                  "api_version": {
                                    "type": "string"
                                  },
                                  "kind": {
                                    "type": "string"
                                  },
                                  "name": {
                                    "type": "string"
                                  }
                                },
                                "required": [
                                  "api_version",
                                  "kind",
                                  "name"
                                ]
                              }
                            },
                            "required": [
                              "name",
                            ]
                          }
                        },
                        "required": [
                          "api_version",
                          "kind",
                          "metadata"
                        ]
                      }
                )),
            }],
        }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
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

/// A ResourceDefinition outlines how a resource is created
#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceDefinition {
    pub group: String,
    pub kind: String,
    pub versions: Vec<ResourceVersion>,
}

impl ResourceDefinition {
    const API_VERSION: &'static str = "core/v1";
    const KIND: &'static str = "resourcedefinition";
}

impl Resource for ResourceDefinition {
    fn resource_ref(&self) -> Ref {
        Ref {
            api_version: ResourceDefinition::API_VERSION.to_string(),
            kind: ResourceDefinition::KIND.to_string(),
            name: "root".to_string(), // TODO
        }
    }
}

/// ResourceVersions capture the different schema between versions
#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceVersion {
    pub schema: ResourceSchema,
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
        let versions = project_def.versions;
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
