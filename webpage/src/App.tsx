import React, { useState, useRef, useEffect } from 'react';
import { Brain, Send, Bot, Loader2 } from 'lucide-react';
import { sendMessageToLLM, Message } from './api/llm';

function App() {
  const [inputValue, setInputValue] = useState('');
  const [conversation, setConversation] = useState<Message[]>([]);
  const [isTyping, setIsTyping] = useState(false);
  const [sessionId, setSessionId] = useState<string | undefined>();
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [conversation]);

  const handleSendMessage = async () => {
    if (inputValue.trim() === '' || isTyping) return;
    
    const newMessage = { role: 'user' as const, content: inputValue };
    const newConversation = [...conversation, newMessage];
    
    setConversation(newConversation);
    setInputValue('');
    setIsTyping(true);
    
    try {
      const response = await sendMessageToLLM(inputValue, conversation, sessionId);
      setIsTyping(false);
      setSessionId(response.sessionId);
      setConversation([
        ...newConversation,
        { role: 'assistant' as const, content: response.content }
      ]);
    } catch (error) {
      console.error('Error getting LLM response:', error);
      setIsTyping(false);
      setConversation([
        ...newConversation,
        { role: 'assistant' as const, content: 'Sorry, I encountered an error while processing your request.' }
      ]);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <div className="min-h-screen bg-[#111111] text-white flex flex-col">
      {/* Header */}
      <div className="border-b border-gray-800 p-4">
        <div className="flex items-center justify-center gap-2">
          <Brain className="h-7 w-7 text-blue-400" />
          <h1 className="text-4xl font-bold text-white">AI NETWORK</h1>
        </div>
        {sessionId && (
          <div className="text-center mt-2 text-gray-400 text-sm">
            Session ID: {sessionId}
          </div>
        )}
      </div>

      {/* Main Chat Area */}
      <div className="flex-1 overflow-y-auto">
        <div className="max-w-3xl mx-auto p-4 md:p-6 space-y-6 mb-24">
          {conversation.length === 0 ? (
            <div className="mt-12">
              <p className="text-gray-400 text-center">
                Start a conversation with your AI assistant
              </p>
            </div>
          ) : (
            conversation.map((message, index) => (
              <div
                key={index}
                className="flex items-start gap-3 bg-[#1a1a1a] p-4 rounded-lg"
              >
                {message.role === "assistant" ? (
                  <Bot className="w-6 h-6 text-blue-400 mt-1" />
                ) : (
                  <div className="w-6 h-6 rounded-full bg-gray-700 flex items-center justify-center">
                    <span className="text-xs text-white">You</span>
                  </div>
                )}
                <div className="flex-1">
                  <p className="text-white whitespace-pre-wrap">{message.content}</p>
                </div>
              </div>
            ))
          )}
          
          {isTyping && (
            <div className="flex items-start gap-3 bg-[#1a1a1a] p-4 rounded-lg">
              <Bot className="w-6 h-6 text-blue-400 mt-1" />
              <div className="flex items-center gap-2">
                <Loader2 className="w-4 h-4 text-gray-400 animate-spin" />
                <span className="text-gray-400 text-sm">Thinking...</span>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>
      </div>

      {/* Input Area */}
      <div className="sticky bottom-0 w-full bg-[#111111] border-t border-gray-800 py-6">
        <form onSubmit={(e) => { e.preventDefault(); handleSendMessage(); }} className="max-w-2xl mx-auto px-4 relative flex">
          <input
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Ask anything..."
            className="w-full bg-[#1a1a1a] border border-gray-800 text-white px-4 py-2 pr-12 rounded-full focus:outline-none focus:border-blue-500"
          />
          <button
            type="submit"
            disabled={isTyping || !inputValue.trim()}
            className="absolute right-6 top-1/2 -translate-y-1/2 h-8 w-8 flex items-center justify-center bg-blue-500 hover:bg-blue-600 disabled:opacity-50 disabled:hover:bg-blue-500 rounded-full"
          >
            <Send className="h-4 w-4" />
          </button>
        </form>
      </div>
    </div>
  );
}

export default App;