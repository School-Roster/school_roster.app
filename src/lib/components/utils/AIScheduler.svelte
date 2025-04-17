<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  
  let messages: {role: 'user' | 'assistant', content: string}[] = [];
  let inputMessage: string = '';
  let isLoading: boolean = false;
  let modelLoaded: boolean = false;
  
  onMount(async () => {
    try {
      isLoading = true;
      await invoke('initialize_model');
      modelLoaded = true;
      messages = [...messages, {
        role: 'assistant',
        content: 'AI assistant loaded. How can I help with your school schedule?'
      }];
    } catch (e) {
      console.error('Failed to load model:', e);
      messages = [...messages, {
        role: 'assistant',
        content: 'Error loading AI model. Please check your installation.'
      }];
    } finally {
      isLoading = false;
    }
  });
  
  async function sendMessage() {
    if (!inputMessage.trim() || isLoading) return;
    
    const userMessage = inputMessage;
    messages = [...messages, { role: 'user', content: userMessage }];
    inputMessage = '';
    isLoading = true;
    
    try {
      const response = await invoke('query_ai', { message: userMessage });
      messages = [...messages, { role: 'assistant', content: response as string }];
    } catch (e) {
      console.error('AI query failed:', e);
      messages = [...messages, {
        role: 'assistant',
        content: 'Sorry, I encountered an error processing your request.'
      }];
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="chat-container">
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
  </div>
  
  <div class="input-area">
    <input 
      type="text" 
      bind:value={inputMessage} 
      placeholder="Ask about scheduling..." 
      on:keydown={(e) => e.key === 'Enter' && sendMessage()}
      disabled={!modelLoaded || isLoading}
    />
    <button on:click={sendMessage} disabled={!modelLoaded || isLoading}>
      Send
    </button>
  </div>
</div>

<style>
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
</style>
