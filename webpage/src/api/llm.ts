import axios from 'axios';

const API_BASE_URL = 'http://localhost:8080';

export interface Message {
  role: 'user' | 'assistant';
  content: string;
}

interface ChatResponse {
  message: Message;
  session_id: string;
}

export async function sendMessageToLLM(
  message: string, 
  conversation: Message[] = [], 
  sessionId?: string
): Promise<{ content: string; sessionId: string }> {
  try {
    const response = await axios.post<ChatResponse>(`${API_BASE_URL}/chat`, {
      message,
      conversation,
      session_id: sessionId,
    });
    
    return {
      content: response.data.message.content,
      sessionId: response.data.session_id,
    };
  } catch (error) {
    console.error('Error sending message to LLM:', error);
    throw new Error('Failed to get response from LLM');
  }
}