use actix_web::{post, web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use reqwest::Client;

const OLLAMA_HOST: &str = "http://127.0.0.1:11434";

#[derive(Deserialize)]
pub struct ChatRequest {
    message: String,
    conversation: Vec<Message>,
    context: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    context: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    context: Option<Vec<i32>>,
}

#[post("/chat")]
pub async fn chat(req: web::Json<ChatRequest>) -> Result<HttpResponse, Error> {
    let client = Client::new();
    
    // Format the conversation history into a single prompt
    let mut full_prompt = String::new();
    for msg in &req.conversation {
        full_prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
    }
    full_prompt.push_str(&format!("user: {}\n", req.message));

    let ollama_req = OllamaRequest {
        model: "qwen2.5-coder:7b".to_string(),
        prompt: full_prompt,
        stream: false,
        context: req.context.clone().unwrap_or_default(),
    };

    let response = client
        .post(format!("{}/api/generate", OLLAMA_HOST))
        .json(&ollama_req)
        .send()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let ollama_response = response
        .json::<OllamaResponse>()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": Message {
            role: "assistant".to_string(),
            content: ollama_response.response,
        },
        "context": ollama_response.context
    })))
}