// src/cdn_app_backend/lib.rs

use candid::Principal;
use ic_cdk::api;
use ic_cdk_macros::{init, query, update};

mod auth;
mod config;
mod file_store;
mod types;
mod utils;

use crate::auth::{add_role, remove_role, get_roles, is_admin};
use crate::file_store::{upload_file, delete_file, download_file, list_files};
use crate::types::{ApiResponse, Role, FileMetadata};

/// Canister initialization
#[init]
fn init() {
    let caller = api::msg_caller();
    ic_cdk::println!("Initializing CDN canister. Admin = {:?}", caller);
    add_role(caller, Role::Admin); // whoever deploys becomes admin
}

//
// ========== AUTH MANAGEMENT ==========
//

/// Add a role to a principal (Admin only)
#[update]
fn assign_role(user: Principal, role: Role) -> ApiResponse<String> {
    if is_admin(api::msg_caller()) {
        add_role(user, role);
        ApiResponse::Success(format!("Role {:?} assigned to {}", role, user.to_text()))
    } else {
        ApiResponse::Error("Permission denied: Only admins can assign roles".into())
    }
}

/// Remove a role from a principal (Admin only)
#[update]
fn revoke_role(user: Principal, role: Role) -> ApiResponse<String> {
    if is_admin(api::msg_caller()) {
        remove_role(user, role);
        ApiResponse::Success(format!("Role {:?} removed from {}", role, user.to_text()))
    } else {
        ApiResponse::Error("Permission denied: Only admins can revoke roles".into())
    }
}

/// Get roles of a user (anyone can check)
#[query]
fn get_user_roles(user: Principal) -> ApiResponse<Vec<Role>> {
    ApiResponse::Success(get_roles(user))
}

//
// ========== FILE OPERATIONS ==========
//

/// Upload file
#[update]
fn upload(name: String, content: Vec<u8>) -> ApiResponse<String> {
    upload_file(name, content)
}

/// Delete file
#[update]
fn delete(name: String) -> ApiResponse<String> {
    delete_file(name)
}

/// Download file
#[query]
fn download(name: String) -> ApiResponse<(Vec<u8>, FileMetadata)> {
    download_file(name)
}

/// List files
#[query]
fn list() -> ApiResponse<Vec<FileMetadata>> {
    list_files()
}
