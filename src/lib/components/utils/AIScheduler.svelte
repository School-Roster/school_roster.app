<script lang="ts">
  import { onMount } from 'svelte';
  import { pipeline, env } from '@xenova/transformers';

  let messages: {role: 'user' | 'assistant', content: string}[] = [];
  let inputMessage: string = '';
  let isLoading: boolean = false;
  let modelLoaded: boolean = false;
  let generator: any;
  let loadingStatus: string = 'Initializing...';

  // Configure the model cache directory
  env.localModelPath = './.models'; // Local cache directory
  env.allowRemoteModels = true; // Allow downloading models from HuggingFace

  // Progress callback
  const progressCallback = (progress: { status: string, progress?: number }) => {
    loadingStatus = progress.status;
    if (progress.progress !== undefined) {
      loadingStatus += ` (${Math.round(progress.progress * 100)}%)`;
    }
    console.log(loadingStatus);
  };

  onMount(async () => {
    try {
      isLoading = true;
      messages = [...messages, {
        role: 'assistant',
        content: 'Loading AI model, this might take a few moments...'
      }];

      // Initialize the model with progress callback
      generator = await pipeline('text-generation', 'Xenova/distilgpt2', {
        progress_callback: progressCallback
      });

      modelLoaded = true;
      messages = [...messages, {
        role: 'assistant',
        content: 'AI assistant loaded. How can I help with your school schedule?'
      }];
    } catch (e) {
      console.error('Failed to load model:', e);
      messages = [...messages, {
        role: 'assistant',
        content: `Error loading AI model: ${e.message}`
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
      const systemPrompt = "You are an AI assistant helping with school scheduling. You can modify class schedules, suggest optimal arrangements, and answer questions about the schedule.";
      const fullPrompt = `${systemPrompt}\n\nUser: ${userMessage}\nAssistant:`;
      
      const result = await generator(fullPrompt, {
        max_new_tokens: 100,
        temperature: 0.7,
        num_return_sequences: 1,
        pad_token_id: generator.tokenizer.eos_token_id,
      });

      const response = result[0].generated_text.split('Assistant:')[1]?.trim() || result[0].generated_text;
      messages = [...messages, { role: 'assistant', content: response }];
    } catch (e) {
      console.error('AI query failed:', e);
      messages = [...messages, {
        role: 'assistant',
        content: `Error: ${e.message}`
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
      <div class="loading">
        <span class="loading-text">{loadingStatus}</span>
        <div class="loading-spinner"></div>
      </div>
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
    <button 
      on:click={sendMessage} 
      disabled={!modelLoaded || isLoading}
      class:loading={isLoading}
    >
      {isLoading ? 'Sending...' : 'Send'}
    </button>
  </div>
</div>

<style>
  .chat-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    max-width: 800px;
    margin: 0 auto;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .message {
    margin-bottom: 0.5rem;
    padding: 1rem;
    border-radius: 0.5rem;
    max-width: 80%;
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
    margin-bottom: 0.5rem;
    display: block;
  }
  
  .input-area {
    display: flex;
    padding: 1rem;
    border-top: 1px solid #ddd;
    gap: 0.5rem;
  }
  
  input {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
  }

  input:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }
  
  button {
    padding: 0.75rem 1.5rem;
    background-color: #0066cc;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  button:hover:not(:disabled) {
    background-color: #0052a3;
  }

  button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
  }

  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 1rem;
    color: #666;
  }

  .loading-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid #f3f3f3;
    border-top: 2px solid #0066cc;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>
