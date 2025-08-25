// src/cdn_app_backend/src/types/user.rs

use crate::types::error::DomainError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use candid::CandidType;

/// Roles for the CDN system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, CandidType)]
pub enum Role {
    Admin,
    Publisher,
    Viewer,
}

impl Role {
    /// Validate a role string and convert to Role enum
    pub fn from_str(role_str: &str) -> Result<Self, DomainError> {
        match role_str.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "publisher" => Ok(Role::Publisher),
            "viewer" => Ok(Role::Viewer),
            _ => Err(DomainError::InvalidRole(role_str.to_string())),
        }
    }
}

/// User struct
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: Role,
    pub email: Option<String>,
    pub is_active: bool,
}

impl User {
    /// Create a new user
    pub fn new(username: String, role: Role, email: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            role,
            email,
            is_active: true,
        }
    }

    /// Deactivate user
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Activate user
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Validate the username and email
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.username.trim().is_empty() {
            return Err(DomainError::InvalidInput(
                "Username cannot be empty".to_string(),
            ));
        }
        if let Some(email) = &self.email {
            if !email.contains('@') {
                return Err(DomainError::InvalidInput(
                    "Invalid email format".to_string(),
                ));
            }
        }
        Ok(())
    }
}
