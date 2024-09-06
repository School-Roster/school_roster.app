<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  
  let messages: {role: 'user' | 'assistant', content: string}[] = [];
  let inputMessage: string = '';
  let isLoading: boolean = false;
  let apiKey: string = '';
  let apiKeyValid: boolean = false;
  let selectedModel: 'standard' | 'reasoner' = 'standard';
  let errorMessage: string = '';
  
  async function validateApiKey() {
    if (!apiKey.trim()) {
      errorMessage = 'Please enter an API key';
      return;
    }
    
    try {
      isLoading = true;
      errorMessage = '';
      const response = await invoke('check_api_key', { apiKey });
      apiKeyValid = true;
      await initializeModel();
    } catch (e) {
      console.error('API key validation failed:', e);
      errorMessage = e as string;
      apiKeyValid = false;
    } finally {
      isLoading = false;
    }
  }
  
  async function initializeModel() {
    try {
      isLoading = true;
      errorMessage = '';
      await invoke('init_model', { apiKey });
      messages = [...messages, {
        role: 'assistant',
        content: 'DeepSeek AI assistant connected. How can I help with your school schedule?'
      }];
    } catch (e) {
      console.error('Failed to initialize DeepSeek API:', e);
      errorMessage = e as string;
      apiKeyValid = false;
    } finally {
      isLoading = false;
    }
  }
  
  async function sendMessage() {
    if (!inputMessage.trim() || isLoading || !apiKeyValid) return;
    
    const userMessage = inputMessage;
    messages = [...messages, { role: 'user', content: userMessage }];
    inputMessage = '';
    isLoading = true;
    errorMessage = '';
    
    try {
      // Use the appropriate endpoint based on model selection
      const endpoint = selectedModel === 'standard' ? 'query_ai' : 'query_ai_reasoner';
      const response = await invoke(endpoint, { message: userMessage });
      messages = [...messages, { role: 'assistant', content: response as string }];
    } catch (e) {
      console.error('AI query failed:', e);
      errorMessage = e as string;
      messages = [...messages, {
        role: 'assistant',
        content: 'Sorry, I encountered an error processing your request.'
      }];
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="app-container">
  {#if !apiKeyValid}
    <div class="api-key-setup">
      <h2>Connect to DeepSeek AI</h2>
      <p>Enter your DeepSeek API key to continue</p>
      
      <div class="input-group">
        <input 
          type="password" 
          bind:value={apiKey} 
          placeholder="sk-..." 
          on:keydown={(e) => e.key === 'Enter' && validateApiKey()}
          disabled={isLoading}
        />
        <button on:click={validateApiKey} disabled={isLoading}>
          {isLoading ? 'Verifying...' : 'Connect'}
        </button>
      </div>
      
      {#if errorMessage}
        <div class="error-message">{errorMessage}</div>
      {/if}
    </div>
  {:else}
    <div class="chat-container">
      <div class="model-selector">
        <label>
          <input type="radio" bind:group={selectedModel} value="standard">
          Standard Chat
        </label>
        <label>
          <input type="radio" bind:group={selectedModel} value="reasoner">
          Reasoner (More detailed)
        </label>
      </div>
      
      <div class="messages">
        {#each messages as message}
          <div class="message {message.role}">
            <span class="role">{message.role === 'user' ? 'You' : 'Assistant'}:</span>
            <p>{message.content}</p>
          </div>
        {/each}
        {#if isLoading}
          <div class="loading">Assistant is thinking...</div>
        {/if}
        {#if errorMessage}
          <div class="error-message">{errorMessage}</div>
        {/if}
      </div>
      
      <div class="input-area">
        <input 
          type="text" 
          bind:value={inputMessage} 
          placeholder="Ask about scheduling..." 
          on:keydown={(e) => e.key === 'Enter' && sendMessage()}
          disabled={isLoading}
        />
        <button on:click={sendMessage} disabled={isLoading}>
          Send
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    max-width: 800px;
    margin: 0 auto;
  }
  
  .api-key-setup {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 2rem;
    text-align: center;
  }
  
  .input-group {
    display: flex;
    width: 100%;
    max-width: 500px;
    margin: 1rem 0;
  }
  
  .model-selector {
    display: flex;
    padding: 0.5rem;
    background-color: #f5f5f5;
    border-bottom: 1px solid #ddd;
  }
  
  .model-selector label {
    margin-right: 1rem;
    cursor: pointer;
  }
  
  .chat-container {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  
  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }
  
  .message {
    margin-bottom: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.5rem;
  }
  
  .message.user {
    background-color: #e0f2ff;
    align-self: flex-end;
  }
  
  .message.assistant {
    background-color: #f0f0f0;
    align-self: flex-start;
  }
  
  .role {
    font-weight: bold;
  }
  
  .input-area {
    display: flex;
    padding: 1rem;
    border-top: 1px solid #ddd;
  }
  
  input {
    flex: 1;
    padding: 0.5rem;
    margin-right: 0.5rem;
  }
  
  .error-message {
    color: #e53935;
    margin: 0.5rem 0;
    padding: 0.5rem;
    background-color: #ffebee;
    border-radius: 0.25rem;
    max-width: 100%;
    overflow-wrap: break-word;
  }
  
  .loading {
    padding: 0.5rem;
    font-style: italic;
    color: #666;
  }
</style>
