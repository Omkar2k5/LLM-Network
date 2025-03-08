use actix_web::{post, web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use chrono::Utc;
use crate::utils::{ChatMessage, save_chat_session};

const OLLAMA_HOST: &str = "http://127.0.0.1:11434";

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    message: String,
    conversation: Vec<Message>,
    session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

#[post("/chat")]
pub async fn chat(req: web::Json<ChatRequest>) -> Result<HttpResponse, Error> {
    println!("Received chat request: {:?}", req);
    
    let client = Client::new();
    
    // Format the conversation history into a single prompt
    let mut full_prompt = String::new();
    for msg in &req.conversation {
        full_prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
    }
    full_prompt.push_str(&format!("user: {}\n", req.message));
    
    println!("Sending request to Ollama with prompt: {}", full_prompt);

    let ollama_req = OllamaRequest {
        model: "qwen2.5-coder:7b".to_string(),
        prompt: full_prompt,
        stream: false,
    };

    let response = match client
        .post(format!("{}/api/generate", OLLAMA_HOST))
        .json(&ollama_req)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Error sending request to Ollama: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to connect to LLM service"
            })));
        }
    };

    let ollama_response = match response.json::<OllamaResponse>().await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Error parsing Ollama response: {}", e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to parse LLM response"
            })));
        }
    };

    // Create or update chat session
    let mut session = if let Some(session_id) = req.session_id.as_ref() {
        if let Ok(Some(existing_session)) = crate::utils::load_chat_session(session_id) {
            existing_session
        } else {
            crate::utils::create_new_session()
        }
    } else {
        crate::utils::create_new_session()
    };

    // Add user message
    session.messages.push(ChatMessage {
        role: "user".to_string(),
        content: req.message.clone(),
        timestamp: Utc::now(),
    });

    // Add assistant message
    session.messages.push(ChatMessage {
        role: "assistant".to_string(),
        content: ollama_response.response.clone(),
        timestamp: Utc::now(),
    });

    session.updated_at = Utc::now();

    // Save the session
    if let Err(e) = save_chat_session(&session) {
        eprintln!("Error saving chat session: {}", e);
    }

    println!("Sending response back to client");
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": Message {
            role: "assistant".to_string(),
            content: ollama_response.response,
        },
        "session_id": session.id
    })))
}