use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use std::env;

fn get_conversation_dir() -> PathBuf {
    let exe_dir = env::current_exe()
        .map(|path| path.parent().unwrap_or(Path::new(".")).to_path_buf())
        .unwrap_or_else(|_| PathBuf::from("."));
    exe_dir.join("conversation")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatSession {
    pub id: String,
    pub messages: Vec<ChatMessage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConversationStore {
    pub sessions: Vec<ChatSession>,
}

impl ConversationStore {
    pub fn load() -> Self {
        println!("Loading conversation store...");
        let conversation_dir = get_conversation_dir();
        let file_path = conversation_dir.join("local.json");

        println!("Checking directory: {:?}", conversation_dir);
        if !conversation_dir.exists() {
            println!("Creating directory: {:?}", conversation_dir);
            match fs::create_dir_all(&conversation_dir) {
                Ok(_) => println!("Directory created successfully"),
                Err(e) => {
                    eprintln!("Error creating conversation directory: {}", e);
                    return ConversationStore::default();
                }
            }
        }

        println!("Checking file: {:?}", file_path);
        if !file_path.exists() {
            println!("File doesn't exist, creating new store");
            let store = ConversationStore::default();
            if let Err(e) = store.save() {
                eprintln!("Error creating initial store file: {}", e);
            }
            return store;
        }

        println!("Reading existing file");
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                println!("File content read successfully");
                match serde_json::from_str(&content) {
                    Ok(store) => {
                        println!("JSON parsed successfully");
                        store
                    }
                    Err(e) => {
                        eprintln!("Error parsing conversation file: {}", e);
                        ConversationStore::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading conversation file: {}", e);
                ConversationStore::default()
            }
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        println!("Saving conversation store...");
        let conversation_dir = get_conversation_dir();
        
        println!("Ensuring directory exists: {:?}", conversation_dir);
        if !conversation_dir.exists() {
            fs::create_dir_all(&conversation_dir)?;
        }

        let file_path = conversation_dir.join("local.json");
        println!("Saving to file: {:?}", file_path);
        
        let json = serde_json::to_string_pretty(self)?;
        println!("JSON serialized successfully, length: {}", json.len());
        
        let mut file = File::create(&file_path)?;
        file.write_all(json.as_bytes())?;
        file.sync_all()?; // Ensure the file is written to disk
        
        println!("File saved successfully");
        Ok(())
    }

    pub fn add_or_update_session(&mut self, session: ChatSession) {
        println!("Adding or updating session: {}", session.id);
        if let Some(existing) = self.sessions.iter_mut().find(|s| s.id == session.id) {
            println!("Updating existing session");
            *existing = session;
        } else {
            println!("Adding new session");
            self.sessions.push(session);
        }
        
        match self.save() {
            Ok(_) => println!("Session saved successfully"),
            Err(e) => eprintln!("Error saving conversation store: {}", e),
        }
    }

    pub fn get_session(&self, id: &str) -> Option<&ChatSession> {
        println!("Getting session: {}", id);
        self.sessions.iter().find(|s| s.id == id)
    }
}

pub fn save_chat_session(session: &ChatSession) -> Result<(), std::io::Error> {
    println!("Saving chat session: {}", session.id);
    let mut store = ConversationStore::load();
    store.add_or_update_session(session.clone());
    Ok(())
}

pub fn load_chat_session(session_id: &str) -> Result<Option<ChatSession>, std::io::Error> {
    println!("Loading chat session: {}", session_id);
    let store = ConversationStore::load();
    Ok(store.get_session(session_id).cloned())
}

pub fn create_new_session() -> ChatSession {
    let id = uuid::Uuid::new_v4().to_string();
    println!("Creating new session: {}", id);
    ChatSession {
        id,
        messages: Vec::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

pub fn list_chat_sessions() -> Result<Vec<String>, std::io::Error> {
    println!("Listing all chat sessions");
    let store = ConversationStore::load();
    Ok(store.sessions.iter().map(|s| s.id.clone()).collect())
}