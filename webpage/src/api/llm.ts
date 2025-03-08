import axios from 'axios';

const API_BASE_URL = 'http://localhost:8080';

export interface Message {
  role: 'user' | 'assistant';
  content: string;
}

interface OllamaResponse {
  message: {
    role: string;
    content: string;
  };
  context?: number[];
}

export async function sendMessageToLLM(message: string, conversation: Message[] = []): Promise<string> {
  try {
    const response = await axios.post<OllamaResponse>(`${API_BASE_URL}/chat`, {
      message,
      conversation,
    });
    return response.data.message.content;
  } catch (error) {
    console.error('Error sending message to LLM:', error);
    throw new Error('Failed to get response from LLM');
  }
}