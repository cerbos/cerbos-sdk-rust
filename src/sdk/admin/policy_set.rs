// Copyright 2021-2025 Zenauth Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::Read;
use std::path::Path;

// Error types for policy operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum PolicyError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Policy parsing error: {0}")]
    Parse(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Empty policy set")]
    EmptyPolicySet,
    #[error("Multiple errors: {0:?}")]
    Multiple(Vec<PolicyError>),
}

// Trait that policy builders must implement
pub trait PolicyBuilder {
    fn build(&self) -> Result<Policy, PolicyError>;
}

// Mock policy types - these would be replaced with actual protobuf generated types
#[derive(Debug, Clone)]
pub struct Policy {
    pub api_version: String,
    pub policy_type: PolicyType,
}

#[derive(Debug, Clone)]
pub enum PolicyType {
    ResourcePolicy(ResourcePolicyData),
    PrincipalPolicy(PrincipalPolicyData),
    DerivedRoles(DerivedRolesData),
    ExportConstants(ExportConstantsData),
    ExportVariables(ExportVariablesData),
}

#[derive(Debug, Clone)]
pub struct ResourcePolicyData {
    pub resource: String,
    pub version: String,
    // Add other fields as needed
}

#[derive(Debug, Clone)]
pub struct PrincipalPolicyData {
    pub principal: String,
    pub version: String,
    // Add other fields as needed
}

#[derive(Debug, Clone)]
pub struct DerivedRolesData {
    pub name: String,
    // Add other fields as needed
}

#[derive(Debug, Clone)]
pub struct ExportConstantsData {
    pub name: String,
    // Add other fields as needed
}

#[derive(Debug, Clone)]
pub struct ExportVariablesData {
    pub name: String,
    // Add other fields as needed
}

// Policy builders (these would be separate modules in practice)
pub struct ResourcePolicy {
    pub obj: ResourcePolicyData,
    pub err: Option<PolicyError>,
}

impl ResourcePolicy {
    pub fn new(resource: String, version: String) -> Self {
        Self {
            obj: ResourcePolicyData { resource, version },
            err: None,
        }
    }
}

impl PolicyBuilder for ResourcePolicy {
    fn build(&self) -> Result<Policy, PolicyError> {
        if let Some(err) = &self.err {
            return Err(err.clone());
        }

        Ok(Policy {
            api_version: "api.cerbos.dev/v1".to_string(),
            policy_type: PolicyType::ResourcePolicy(self.obj.clone()),
        })
    }
}

pub struct PrincipalPolicy {
    pub obj: PrincipalPolicyData,
    pub err: Option<PolicyError>,
}

impl PrincipalPolicy {
    pub fn new(principal: String, version: String) -> Self {
        Self {
            obj: PrincipalPolicyData { principal, version },
            err: None,
        }
    }
}

impl PolicyBuilder for PrincipalPolicy {
    fn build(&self) -> Result<Policy, PolicyError> {
        if let Some(err) = &self.err {
            return Err(err.clone());
        }

        Ok(Policy {
            api_version: "api.cerbos.dev/v1".to_string(),
            policy_type: PolicyType::PrincipalPolicy(self.obj.clone()),
        })
    }
}

pub struct DerivedRoles {
    pub obj: DerivedRolesData,
    pub err: Option<PolicyError>,
}

impl DerivedRoles {
    pub fn new(name: String) -> Self {
        Self {
            obj: DerivedRolesData { name },
            err: None,
        }
    }
}

impl PolicyBuilder for DerivedRoles {
    fn build(&self) -> Result<Policy, PolicyError> {
        if let Some(err) = &self.err {
            return Err(err.clone());
        }

        Ok(Policy {
            api_version: "api.cerbos.dev/v1".to_string(),
            policy_type: PolicyType::DerivedRoles(self.obj.clone()),
        })
    }
}

pub struct ExportConstants {
    pub obj: ExportConstantsData,
    pub err: Option<PolicyError>,
}

impl ExportConstants {
    pub fn new(name: String) -> Self {
        Self {
            obj: ExportConstantsData { name },
            err: None,
        }
    }
}

impl PolicyBuilder for ExportConstants {
    fn build(&self) -> Result<Policy, PolicyError> {
        if let Some(err) = &self.err {
            return Err(err.clone());
        }

        Ok(Policy {
            api_version: "api.cerbos.dev/v1".to_string(),
            policy_type: PolicyType::ExportConstants(self.obj.clone()),
        })
    }
}

pub struct ExportVariables {
    pub obj: ExportVariablesData,
    pub err: Option<PolicyError>,
}

impl ExportVariables {
    pub fn new(name: String) -> Self {
        Self {
            obj: ExportVariablesData { name },
            err: None,
        }
    }
}

impl PolicyBuilder for ExportVariables {
    fn build(&self) -> Result<Policy, PolicyError> {
        if let Some(err) = &self.err {
            return Err(err.clone());
        }

        Ok(Policy {
            api_version: "api.cerbos.dev/v1".to_string(),
            policy_type: PolicyType::ExportVariables(self.obj.clone()),
        })
    }
}

/// PolicySet is a container for a set of policies.
pub struct PolicySet {
    err: Vec<PolicyError>,
    policies: Vec<Policy>,
}

impl PolicySet {
    /// Creates a new policy set.
    pub fn new() -> Self {
        Self {
            err: Vec::new(),
            policies: Vec::new(),
        }
    }

    /// Adds a policy from the given file to the set.
    pub fn add_policy_from_file<P: AsRef<Path>>(mut self, file: P) -> Self {
        match File::open(&file) {
            Ok(mut f) => {
                let mut contents = String::new();
                if let Err(e) = f.read_to_string(&mut contents) {
                    self.err.push(PolicyError::Io(e.to_string()));
                } else {
                    self = self.add_policy_from_string(&contents);
                }
            }
            Err(e) => {
                self.err.push(PolicyError::Io(e.to_string()));
            }
        }
        self
    }

    pub fn add_policy_from_file_with_err<P: AsRef<Path>>(
        mut self,
        file: P,
    ) -> Result<Self, PolicyError> {
        let mut f = File::open(file).map_err(|e| PolicyError::Io(e.to_string()))?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .map_err(|e| PolicyError::Io(e.to_string()))?;

        // In a real implementation, this would parse the policy
        // For now, just add a mock policy
        self.policies.push(Policy {
            api_version: "api.cerbos.dev/v1".to_string(),
            policy_type: PolicyType::ResourcePolicy(ResourcePolicyData {
                resource: "from_file".to_string(),
                version: "default".to_string(),
            }),
        });

        Ok(self)
    }

    /// Adds a policy from the given string content to the set.
    pub fn add_policy_from_string(mut self, content: &str) -> Self {
        // In a real implementation, this would parse the YAML/JSON
        // For now, just add a mock policy
        self.policies.push(Policy {
            api_version: "api.cerbos.dev/v1".to_string(),
            policy_type: PolicyType::ResourcePolicy(ResourcePolicyData {
                resource: "from_string".to_string(),
                version: "default".to_string(),
            }),
        });
        self
    }

    /// Adds the given policies to the set.
    pub fn add_policies(mut self, policies: Vec<Policy>) -> Self {
        self.policies.extend(policies);
        self
    }

    /// Adds the given resource policies to the set.
    pub fn add_resource_policies(mut self, policies: Vec<ResourcePolicy>) -> Self {
        for policy in policies {
            match self.add_policy_builder(policy) {
                Ok(p) => self.policies.push(p),
                Err(e) => {
                    let resource = match e {
                        PolicyError::Validation(ref msg) if msg.contains("resource") => {
                            "unknown".to_string()
                        }
                        _ => "unknown".to_string(),
                    };
                    self.err.push(PolicyError::Parse(format!(
                        "failed to add resource policy [{}:unknown]: {}",
                        resource, e
                    )));
                }
            }
        }
        self
    }

    /// Adds the given principal policies to the set.
    pub fn add_principal_policies(mut self, policies: Vec<PrincipalPolicy>) -> Self {
        for policy in policies {
            match self.add_policy_builder(policy) {
                Ok(p) => self.policies.push(p),
                Err(e) => {
                    let principal = match e {
                        PolicyError::Validation(ref msg) if msg.contains("principal") => {
                            "unknown".to_string()
                        }
                        _ => "unknown".to_string(),
                    };
                    self.err.push(PolicyError::Parse(format!(
                        "failed to add principal policy [{}:unknown]: {}",
                        principal, e
                    )));
                }
            }
        }
        self
    }

    /// Adds the given derived roles to the set.
    pub fn add_derived_roles(mut self, policies: Vec<DerivedRoles>) -> Self {
        for policy in policies {
            match self.add_policy_builder(policy) {
                Ok(p) => self.policies.push(p),
                Err(e) => {
                    let name = policy.obj.name.clone();
                    self.err.push(PolicyError::Parse(format!(
                        "failed to add derived roles [{}]: {}",
                        name, e
                    )));
                }
            }
        }
        self
    }

    /// Adds the given exported constants to the set.
    pub fn add_export_constants(mut self, policies: Vec<ExportConstants>) -> Self {
        for policy in policies {
            match self.add_policy_builder(policy) {
                Ok(p) => self.policies.push(p),
                Err(e) => {
                    let name = policy.obj.name.clone();
                    self.err.push(PolicyError::Parse(format!(
                        "failed to add exported constants [{}]: {}",
                        name, e
                    )));
                }
            }
        }
        self
    }

    /// Adds the given exported variables to the set.
    pub fn add_export_variables(mut self, policies: Vec<ExportVariables>) -> Self {
        for policy in policies {
            match self.add_policy_builder(policy) {
                Ok(p) => self.policies.push(p),
                Err(e) => {
                    let name = policy.obj.name.clone();
                    self.err.push(PolicyError::Parse(format!(
                        "failed to add exported variables [{}]: {}",
                        name, e
                    )));
                }
            }
        }
        self
    }

    /// Returns all of the policies in the set.
    pub fn get_policies(&self) -> &[Policy] {
        &self.policies
    }

    /// Returns the number of policies in this set.
    pub fn size(&self) -> usize {
        self.policies.len()
    }

    /// Returns the errors accumulated during the construction of the policy set.
    pub fn err(&self) -> Option<PolicyError> {
        if self.err.is_empty() {
            None
        } else if self.err.len() == 1 {
            Some(self.err[0].clone())
        } else {
            Some(PolicyError::Multiple(self.err.clone()))
        }
    }

    /// Checks whether the policy set is valid.
    pub fn validate(&self) -> Result<(), PolicyError> {
        if let Some(err) = self.err() {
            return Err(err);
        }

        if self.policies.is_empty() {
            return Err(PolicyError::EmptyPolicySet);
        }

        Ok(())
    }

    /// Helper method to add a policy builder to the set.
    fn add_policy_builder<T: PolicyBuilder>(&self, builder: T) -> Result<Policy, PolicyError> {
        builder.build()
    }
}

impl Default for PolicySet {
    fn default() -> Self {
        Self::new()
    }
}

// Builder pattern methods that return Self for chaining
impl PolicySet {
    /// Builder-style method for adding resource policies.
    pub fn with_resource_policies(self, policies: Vec<ResourcePolicy>) -> Self {
        self.add_resource_policies(policies)
    }

    /// Builder-style method for adding principal policies.
    pub fn with_principal_policies(self, policies: Vec<PrincipalPolicy>) -> Self {
        self.add_principal_policies(policies)
    }

    /// Builder-style method for adding derived roles.
    pub fn with_derived_roles(self, policies: Vec<DerivedRoles>) -> Self {
        self.add_derived_roles(policies)
    }

    /// Builder-style method for adding export constants.
    pub fn with_export_constants(self, policies: Vec<ExportConstants>) -> Self {
        self.add_export_constants(policies)
    }

    /// Builder-style method for adding export variables.
    pub fn with_export_variables(self, policies: Vec<ExportVariables>) -> Self {
        self.add_export_variables(policies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_policy_set() {
        let ps = PolicySet::new();
        assert_eq!(ps.size(), 0);
        assert!(ps.err().is_none());
    }

    #[test]
    fn test_add_resource_policies() {
        let resource_policy = ResourcePolicy::new("document".to_string(), "v1".to_string());
        let ps = PolicySet::new().add_resource_policies(vec![resource_policy]);

        assert_eq!(ps.size(), 1);
        assert!(ps.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_policy_set() {
        let ps = PolicySet::new();
        assert!(matches!(ps.validate(), Err(PolicyError::EmptyPolicySet)));
    }

    #[test]
    fn test_builder_pattern() {
        let resource_policy = ResourcePolicy::new("document".to_string(), "v1".to_string());
        let principal_policy = PrincipalPolicy::new("user".to_string(), "v1".to_string());

        let ps = PolicySet::new()
            .with_resource_policies(vec![resource_policy])
            .with_principal_policies(vec![principal_policy]);

        assert_eq!(ps.size(), 2);
        assert!(ps.validate().is_ok());
    }
}
