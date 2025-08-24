// src/cdn_app_backend/file_store.rs

use crate::auth::{require_permission, Permission};
use crate::types::{ApiResponse, FileMetadata};
use candid::Principal;
use ic_cdk::caller;
use ic_cdk::api::time;
use std::collections::HashMap;
use std::cell::RefCell;

/// Internal storage (in-memory for canister)
thread_local! {
    static FILES: RefCell<HashMap<String, (Vec<u8>, FileMetadata)>> = RefCell::new(HashMap::new());
}

/// Upload a file (Admins & Uploaders allowed)
pub fn upload_file(name: String, content: Vec<u8>) -> ApiResponse<String> {
    match require_permission(msg_caller(), Permission::UploadFile) {
        Ok(_) => {
            let metadata = FileMetadata {
                name: name.clone(),
                size: content.len() as u64,
                uploaded_by: msg_caller().to_text(),
                uploaded_at: time(),
            };

            FILES.with(|files| {
                files.borrow_mut().insert(name.clone(), (content, metadata));
            });

            ApiResponse::Success(format!("File '{}' uploaded successfully.", name))
        }
        Err(err) => ApiResponse::Error(err),
    }
}

/// Delete a file (Admins only)
pub fn delete_file(name: String) -> ApiResponse<String> {
    match require_permission(msg_caller(), Permission::DeleteFile) {
        Ok(_) => {
            FILES.with(|files| {
                let mut files = files.borrow_mut();
                if files.remove(&name).is_some() {
                    ApiResponse::Success(format!("File '{}' deleted successfully.", name))
                } else {
                    ApiResponse::Error(format!("File '{}' not found.", name))
                }
            })
        }
        Err(err) => ApiResponse::Error(err),
    }
}

/// Download a file (Admins, Uploaders, Viewers allowed)
pub fn download_file(name: String) -> ApiResponse<(Vec<u8>, FileMetadata)> {
    match require_permission(msg_caller(), Permission::DownloadFile) {
        Ok(_) => {
            FILES.with(|files| {
                let files = files.borrow();
                if let Some((content, metadata)) = files.get(&name) {
                    ApiResponse::Success((content.clone(), metadata.clone()))
                } else {
                    ApiResponse::Error(format!("File '{}' not found.", name))
                }
            })
        }
        Err(err) => ApiResponse::Error(err),
    }
}

/// List all files (Admins, Uploaders, Viewers allowed)
pub fn list_files() -> ApiResponse<Vec<FileMetadata>> {
    match require_permission(msg_caller(), Permission::ListFiles) {
        Ok(_) => {
            FILES.with(|files| {
                let files = files.borrow();
                let metadata_list = files.values().map(|(_, meta)| meta.clone()).collect();
                ApiResponse::Success(metadata_list)
            })
        }
        Err(err) => ApiResponse::Error(err),
    }
}
