// src/cdn_app_backend/auth.rs

use ic_cdk::api::caller::msg_caller;
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::types::{Role, Permission};

thread_local! {
    // Stores roles for each user
    static USER_ROLES: RefCell<HashMap<candid::Principal, Vec<Role>>> =
        RefCell::new(HashMap::new());
}

/// Initialize the first admin during canister setup.
/// Can only be called once.
#[update]
pub fn init_admin(user: candid::Principal) -> Result<(), String> {
    USER_ROLES.with(|roles| {
        let mut map = roles.borrow_mut();
        if map.values().any(|v| v.contains(&Role::Admin)) {
            return Err("Admin already initialized".to_string());
        }
        map.entry(user).or_insert_with(Vec::new).push(Role::Admin);
        Ok(())
    })
}

/// Add a role to a user (Admin only)
#[update]
pub fn add_role(user: candid::Principal, role: Role) -> Result<(), String> {
    let caller = msg_caller();
    ensure_admin(&caller)?;

    USER_ROLES.with(|roles| {
        let mut map = roles.borrow_mut();
        let entry = map.entry(user).or_insert_with(Vec::new);

        if entry.contains(&role) {
            return Err(format!("User {:?} already has role {:?}", user, role));
        }

        entry.push(role);
        Ok(())
    })
}

/// Remove a role from a user (Admin only)
#[update]
pub fn remove_role(user: candid::Principal, role: Role) -> Result<(), String> {
    let caller = msg_caller();
    ensure_admin(&caller)?;

    USER_ROLES.with(|roles| {
        let mut map = roles.borrow_mut();

        if let Some(existing) = map.get_mut(&user) {
            existing.retain(|r| r != &role);

            // Prevent removing the *last* Admin (canister must always have one)
            if role == Role::Admin && !map.values().any(|v| v.contains(&Role::Admin)) {
                existing.push(Role::Admin);
                return Err("Cannot remove the last Admin".to_string());
            }
        } else {
            return Err("User not found".to_string());
        }
        Ok(())
    })
}

/// Get all roles of a user
#[query]
pub fn get_roles(user: candid::Principal) -> Vec<Role> {
    USER_ROLES.with(|roles| {
        roles
            .borrow()
            .get(&user)
            .cloned()
            .unwrap_or_else(Vec::new)
    })
}

/// Check if a user has the required permission
pub fn check_permission(user: &candid::Principal, permission: Permission) -> Result<(), String> {
    let roles = get_roles(*user);

    match permission {
        Permission::ManageUsers | Permission::DeleteFile => {
            if roles.contains(&Role::Admin) {
                Ok(())
            } else {
                Err("Not authorized: Admin required".to_string())
            }
        }
        Permission::UploadFile => {
            if roles.contains(&Role::Admin) || roles.contains(&Role::Uploader) {
                Ok(())
            } else {
                Err("Not authorized: Uploader or Admin required".to_string())
            }
        }
        Permission::DownloadFile | Permission::ListFiles => {
            if roles.contains(&Role::Admin)
                || roles.contains(&Role::Uploader)
                || roles.contains(&Role::Viewer)
            {
                Ok(())
            } else {
                Err("Not authorized: Viewer, Uploader or Admin required".to_string())
            }
        }
    }
}

/// Helper: ensure caller is an Admin
fn ensure_admin(user: &candid::Principal) -> Result<(), String> {
    let roles = get_roles(*user);
    if roles.contains(&Role::Admin) {
        Ok(())
    } else {
        Err("Only Admins can perform this action".to_string())
    }
}

use ic_cdk::export::Principal;
use crate::config::{Role, Permission};

// Wraps your existing permission check function
pub fn require_permission(user: Principal, permission: Permission) -> Result<(), String> {
    check_permission(&user, permission)
}

// Check if a user has the Admin role
pub fn is_admin(user: Principal) -> bool {
    get_roles(user).contains(&Role::Admin)
}

// Require that a user has a specific role
pub fn require_role(user: &Principal, roles: &[Role]) -> Result<(), String> {
    let user_roles = get_roles(*user);
    if roles.iter().any(|r| user_roles.contains(r)) {
        Ok(())
    } else {
        Err("User does not have required role".to_string())
    }
}


use ic_cdk::export::Principal;
use crate::types::{Role, Permission};

// Add a role to a user
pub fn add_role(user: Principal, role: Role) -> Result<(), String> {
    // Implement your logic here
    Ok(())
}

// Remove a role from a user
pub fn remove_role(user: Principal, role: Role) -> Result<(), String> {
    // Implement your logic here
    Ok(())
}

// Get roles of a user
pub fn get_roles(user: Principal) -> Vec<Role> {
    // Replace with your logic
    vec![]
}

// Check if a user has a permission
pub fn check_permission(user: &Principal, permission: Permission) -> Result<(), String> {
    // Replace with your logic
    Ok(())
}

// Check if a user is admin
pub fn is_admin(user: Principal) -> bool {
    get_roles(user).contains(&Role::Admin)
}

