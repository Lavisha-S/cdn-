use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api, storage, trap};
// use fully-qualified attributes like #[ic_cdk::query] to avoid needing
// separate imports for the attribute macros.
use serde::{Deserialize as SerdeDeserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Constants
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const SESSION_DURATION: u64 = 24 * 60 * 60; // 24 hours in seconds
const ADMIN_PRINCIPAL_ID: &str = "2vxsx-fae"; // Replace with your principal ID

// Error Types
#[derive(Debug, Clone, CandidType, Serialize, SerdeDeserialize)]
pub enum DomainError {
    InvalidInput(String),
    InvalidRole(String),
    InvalidState(String),
    Forbidden(String),
    Unauthorized(String),
    NotFound(String),
    InvalidData(String),
    DuplicateEntry(String),
    DataCorruption(String),
    ServiceUnavailable(String),
    ConfigError(String),
    LimitExceeded(String),
    Other(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            DomainError::InvalidRole(msg) => write!(f, "Invalid role: {}", msg),
            DomainError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            DomainError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            DomainError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            DomainError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DomainError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            DomainError::DuplicateEntry(msg) => write!(f, "Duplicate entry: {}", msg),
            DomainError::DataCorruption(msg) => write!(f, "Data corruption: {}", msg),
            DomainError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
            DomainError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            DomainError::LimitExceeded(msg) => write!(f, "Limit exceeded: {}", msg),
            DomainError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

#[derive(Debug, Clone, CandidType, Serialize, SerdeDeserialize)]
pub enum BackendError {
    NetworkError(String),
    StorageUnavailable(String),
    UnexpectedFailure(String),
    DomainError(String),
    Unauthorized(String),
    Forbidden(String),
    InvalidData(String),
    ValidationError(String),
    DuplicateFile,
    FileNotFound,
    Other(String),
}

// User Types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, SerdeDeserialize, CandidType)]
pub enum Role {
    Admin,
    Publisher,
    Viewer,
}

#[derive(Debug, Clone, Serialize, SerdeDeserialize, CandidType)]
pub struct User {
    pub id: String,
    pub username: String,
    pub role: Role,
    pub email: Option<String>,
    pub is_active: bool,
}

// File Types
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub struct FileMetadata {
    pub id: String,
    pub owner: Principal,
    pub owner_id: String,
    pub filename: String,
    pub size: u64,
    pub mime_type: String,
    pub uploaded_at: u64,
    pub roles_allowed: Vec<Role>,
    pub chunk_count: u32,
    pub is_active: bool,
    pub file_hash: Option<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub struct FileChunk {
    pub file_id: String,
    pub index: u32,
    pub data: Vec<u8>,
}

// State Management
thread_local! {
    static STATE: std::cell::RefCell<State> = std::cell::RefCell::new(State::default());
}

#[derive(Clone, Debug, CandidType, Serialize, SerdeDeserialize)]
pub struct Session {
    pub user_id: Principal,
    pub expires_at: u64,
    pub roles: Vec<Role>,
}

struct State {
    files: HashMap<String, FileMetadata>,
    chunks: HashMap<String, Vec<FileChunk>>,
    users: HashMap<Principal, User>,
    roles: HashMap<Principal, Vec<Role>>,
    sessions: HashMap<String, Session>,
    config: Config,
    id_counter: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            files: HashMap::new(),
            chunks: HashMap::new(),
            users: HashMap::new(),
            roles: HashMap::new(),
            sessions: HashMap::new(),
            config: Config::default(),
            id_counter: 0,
        }
    }
}

#[derive(Clone, CandidType, Deserialize)]
struct Config {
    max_file_size_bytes: u64,
    uploads_enabled: bool,
    cdn_domain: Option<String>,
    last_updated_nanos: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_file_size_bytes: 10 * 1024 * 1024, // 10MB
            uploads_enabled: true,
            cdn_domain: None,
            last_updated_nanos: ic_cdk::api::time(),
        }
    }
}

// API Types
type ResultText = Result<String, String>;
type ResultFile = Result<FileContents, String>;
type ResultFileInfoVec = Result<Vec<FileInfo>, String>;
type ResultConfig = Result<Config, String>;
type ResultRoleVec = Result<Vec<Role>, String>;
type ResultRoleMap = Result<Vec<(String, Vec<Role>)>, String>;

#[derive(CandidType, Deserialize)]
struct FileInfo {
    id: String,
    filename: String,
    uploader: String,
    uploaded_at: u64,
}

#[derive(CandidType, Deserialize)]
struct FileContents {
    filename: String,
    content: Vec<u8>,
}

// Helper Functions
fn get_caller_id() -> Principal {
    api::caller()
}

fn get_current_time() -> u64 {
    api::time()
}

fn create_session(user_id: Principal, roles: Vec<Role>) -> Session {
    Session {
        user_id,
        expires_at: get_current_time() + SESSION_DURATION * 1_000_000_000, // Convert to nanoseconds
        roles,
    }
}

fn generate_id() -> String {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.id_counter = s.id_counter.saturating_add(1);
        let now = api::time();
        let raw = format!("{}:{}", now, s.id_counter);
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        hex::encode(hasher.finalize())
    })
}

fn validate_session(session_id: &str) -> Option<Session> {
    STATE.with(|state| {
        let state = state.borrow();
        state.sessions.get(session_id).and_then(|session| {
            if session.expires_at > get_current_time() {
                Some(session.clone())
            } else {
                None
            }
        })
    })
}

fn check_auth(required_role: Option<Role>) -> Result<Session, String> {
    let caller = get_caller_id();
    
    STATE.with(|state| {
        let state = state.borrow();
        
        // Check if user exists and has roles
        let user_roles = state.roles.get(&caller)
            .ok_or_else(|| "User not found".to_string())?;
        
        // If a specific role is required, check for it
        if let Some(role) = required_role {
            if !user_roles.contains(&role) {
                return Err(format!("Required role {:?} not found", role));
            }
        }
        
        Ok(create_session(caller, user_roles.clone()))
    })
}

fn check_admin() -> Result<Session, String> {
    check_auth(Some(Role::Admin))
}

fn hash_data(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

#[ic_cdk::query(name = "verify_session")]
fn verify_session(session_id: String) -> Result<Session, String> {
    validate_session(&session_id)
        .ok_or_else(|| "Invalid or expired session".to_string())
}

#[ic_cdk::update(name = "login")]
fn login() -> Result<(String, Session), String> {
    let caller = get_caller_id();
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Ensure user exists
        if !state.users.contains_key(&caller) {
            return Err("User not registered".to_string());
        }
        
        // Create new session
        let session = create_session(
            caller,
            state.roles.get(&caller).cloned().unwrap_or_default(),
        );
        
    // Generate session ID
    let session_id = generate_id();
        
        // Store session
        state.sessions.insert(session_id.clone(), session.clone());
        
        Ok((session_id, session))
    })
}

#[ic_cdk::update(name = "register")]
fn register(username: String, email: Option<String>) -> Result<User, String> {
    let caller = get_caller_id();
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Check if user already exists
        if state.users.contains_key(&caller) {
            return Err("User already registered".to_string());
        }
        
        // Create new user
        let user = User {
            id: caller.to_string(),
            username,
            role: Role::Viewer, // Default role
            email,
            is_active: true,
        };
        
        // Store user and default role
        state.users.insert(caller, user.clone());
        state.roles.insert(caller, vec![Role::Viewer]);
        
        Ok(user)
    })
}

#[ic_cdk::update(name = "logout")]
fn logout(session_id: String) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.sessions.remove(&session_id);
        Ok(())
    })
}

// Canister API Implementation
#[ic_cdk::init]
fn init() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let admin = get_caller_id();
        state.roles.insert(admin, vec![Role::Admin]);
    });
}

#[ic_cdk::query(name = "health")]
fn health() -> String {
    "healthy".to_string()
}

#[ic_cdk::query(name = "stats")]
fn stats() -> String {
    STATE.with(|state| {
        let state = state.borrow();
        format!(
            "Files: {}, Users: {}, Active uploads enabled: {}",
            state.files.len(),
            state.users.len(),
            state.config.uploads_enabled
        )
    })
}

#[ic_cdk::update(name = "upload_file")]
fn upload_file(filename: String, content: Vec<u8>) -> ResultText {
    let caller = get_caller_id();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if !state.config.uploads_enabled {
            return Err("Uploads are currently disabled".to_string());
        }
        
        if content.len() as u64 > state.config.max_file_size_bytes {
            return Err("File size exceeds maximum allowed".to_string());
        }

    let file_id = generate_id();
        let file_hash = hash_data(&content);
        
        let metadata = FileMetadata {
            id: file_id.clone(),
            owner: caller,
            owner_id: caller.to_string(),
            filename: filename.clone(),
            size: content.len() as u64,
            mime_type: "application/octet-stream".to_string(),
            uploaded_at: api::time(),
            roles_allowed: vec![Role::Admin, Role::Publisher, Role::Viewer],
            chunk_count: 1,
            is_active: true,
            file_hash: Some(file_hash),
        };

        let chunk = FileChunk {
            file_id: file_id.clone(),
            index: 0,
            data: content,
        };

        state.files.insert(file_id.clone(), metadata);
        state.chunks.insert(file_id.clone(), vec![chunk]);

        Ok(file_id)
    })
}

#[ic_cdk::query(name = "get_file")]
fn get_file(file_id: String) -> ResultFile {
    let caller = get_caller_id();
    STATE.with(|state| {
        let state = state.borrow();
        
        if let Some(metadata) = state.files.get(&file_id) {
            if !metadata.is_active {
                return Err("File is not active".to_string());
            }

            let has_access = state.roles.get(&caller)
                .map(|roles| roles.iter().any(|role| metadata.roles_allowed.contains(role)))
                .unwrap_or(false);

            if !has_access && metadata.owner != caller {
                return Err("Access denied".to_string());
            }

            if let Some(chunks) = state.chunks.get(&file_id) {
                let content = chunks.iter()
                    .flat_map(|chunk| chunk.data.clone())
                    .collect();

                Ok(FileContents {
                    filename: metadata.filename.clone(),
                    content,
                })
            } else {
                Err("File content not found".to_string())
            }
        } else {
            Err("File not found".to_string())
        }
    })
}

#[ic_cdk::query(name = "list_files")]
fn list_files() -> ResultFileInfoVec {
    let caller = get_caller_id();
    STATE.with(|state| {
        let state = state.borrow();
        let user_roles = state.roles.get(&caller);
        
        let files: Vec<FileInfo> = state.files.values()
            .filter(|metadata| {
                metadata.is_active && (
                    metadata.owner == caller ||
                    user_roles.map(|roles| roles.iter().any(|role| metadata.roles_allowed.contains(role))).unwrap_or(false)
                )
            })
            .map(|metadata| FileInfo {
                id: metadata.id.clone(),
                filename: metadata.filename.clone(),
                uploader: metadata.owner_id.clone(),
                uploaded_at: metadata.uploaded_at,
            })
            .collect();

        Ok(files)
    })
}

#[ic_cdk::update(name = "delete_file")]
fn delete_file(file_id: String) -> ResultText {
    let caller = get_caller_id();
    STATE.with(|state| {
        // To avoid simultaneous immutable + mutable borrows of state,
        // do a short immutable borrow first to check access, then perform
        // the mutable operations in a separate scope.
        {
            let state_ref = state.borrow();
            if !state_ref.files.contains_key(&file_id) {
                return Err("File not found".to_string());
            }
            let metadata = state_ref.files.get(&file_id).unwrap();
            if metadata.owner != caller && !state_ref.roles.get(&caller).map(|roles| roles.contains(&Role::Admin)).unwrap_or(false) {
                return Err("Access denied".to_string());
            }
        }

        // Now take a mutable borrow and perform the update
        let mut state_mut = state.borrow_mut();
        if let Some(metadata) = state_mut.files.get_mut(&file_id) {
            metadata.is_active = false;
            state_mut.chunks.remove(&file_id);
            Ok("File deleted successfully".to_string())
        } else {
            Err("File not found".to_string())
        }
    })
}

#[ic_cdk::update(name = "wipe_all")]
fn wipe_all() -> ResultText {
    check_admin()?;
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.files.clear();
        state.chunks.clear();
        Ok("All files wiped successfully".to_string())
    })
}

#[ic_cdk::query(name = "whoami")]
fn whoami() -> (String, Vec<Role>) {
    let caller = get_caller_id();
    STATE.with(|state| {
        let state = state.borrow();
        (
            caller.to_string(),
            state.roles.get(&caller).cloned().unwrap_or_default()
        )
    })
}

#[ic_cdk::update(name = "grant_role")]
fn grant_role(principal_text: String, role: Role) -> ResultRoleVec {
    check_admin()?;
    
    let principal = Principal::from_text(principal_text)
        .map_err(|e| format!("Invalid principal: {}", e))?;

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let roles = state.roles.entry(principal).or_insert_with(Vec::new);
        if !roles.contains(&role) {
            roles.push(role);
        }
        Ok(roles.clone())
    })
}

#[ic_cdk::update(name = "revoke_role")]
fn revoke_role(principal_text: String, role: Role) -> ResultRoleVec {
    check_admin()?;
    
    let principal = Principal::from_text(principal_text)
        .map_err(|e| format!("Invalid principal: {}", e))?;

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(roles) = state.roles.get_mut(&principal) {
            roles.retain(|r| r != &role);
            Ok(roles.clone())
        } else {
            Ok(vec![])
        }
    })
}

#[ic_cdk::query(name = "list_roles_of")]
fn list_roles_of(principal_text: String) -> ResultRoleVec {
    check_admin()?;
    
    let principal = Principal::from_text(principal_text)
        .map_err(|e| format!("Invalid principal: {}", e))?;

    STATE.with(|state| {
        let state = state.borrow();
        Ok(state.roles.get(&principal).cloned().unwrap_or_default())
    })
}

#[ic_cdk::query(name = "list_all_user_roles")]
fn list_all_user_roles() -> ResultRoleMap {
    check_admin()?;
    
    STATE.with(|state| {
        let state = state.borrow();
        let roles_map: Vec<(String, Vec<Role>)> = state.roles
            .iter()
            .map(|(principal, roles)| (principal.to_string(), roles.clone()))
            .collect();
        Ok(roles_map)
    })
}

#[ic_cdk::query(name = "get_config")]
fn get_config() -> Config {
    STATE.with(|state| state.borrow().config.clone())
}

#[ic_cdk::update(name = "update_config")]
fn update_config(
    max_file_size_bytes: Option<u64>,
    uploads_enabled: Option<bool>,
    cdn_domain: Option<Option<String>>,
) -> ResultConfig {
    check_admin()?;
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if let Some(size) = max_file_size_bytes {
            state.config.max_file_size_bytes = size;
        }
        if let Some(enabled) = uploads_enabled {
            state.config.uploads_enabled = enabled;
        }
        if let Some(domain) = cdn_domain {
            state.config.cdn_domain = domain;
        }
        state.config.last_updated_nanos = api::time();
        Ok(state.config.clone())
    })
}

#[ic_cdk::update(name = "reset_config")]
fn reset_config() -> ResultConfig {
    check_admin()?;
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.config = Config::default();
        Ok(state.config.clone())
    })
}

#[ic_cdk::query]
fn __get_candid_interface_tmp_hack() -> String {
    include_str!("../cdn_app_backend.did").to_string()
}
