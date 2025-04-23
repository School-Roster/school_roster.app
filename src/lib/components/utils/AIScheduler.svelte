<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";

  let messages: { role: "user" | "assistant"; content: string }[] = [];
  let inputMessage: string = "";
  let isLoading: boolean = false;
  let modelLoaded: boolean = false;
  let loadingStatus: string = "Initializing...";

  // Load the model via Rust backend
  async function loadModel() {
    try {
      loadingStatus = "Loading model...";
      
      const response = await invoke("load_model");
      console.log("Model load response:", response);
      
      // Check model status after loading
      const status = await invoke("get_model_status");
      modelLoaded = status.loaded;
      
      if (modelLoaded) {
        return true;
      } else {
        throw new Error(status.message);
      }
    } catch (error) {
      console.error("Error loading model:", error);
      await message(`Failed to load model: ${error}`, {
        title: "Error",
        type: "error",
      });
      return false;
    }
  }

  onMount(async () => {
    try {
      isLoading = true;
      messages = [
        ...messages,
        {
          role: "assistant",
          content: "Loading AI model, this might take a few moments...",
        },
      ];

      const success = await loadModel();

      if (success) {
        messages = [
          ...messages,
          {
            role: "assistant",
            content:
              "AI assistant loaded. How can I help with your school schedule?",
          },
        ];
      } else {
        throw new Error("Failed to load the model");
      }
    } catch (e) {
      console.error("Failed to load model:", e);
      messages = [
        ...messages,
        {
          role: "assistant",
          content: `Error loading AI model: ${e.message}`,
        },
      ];
    } finally {
      isLoading = false;
    }
  });

  async function generateText(prompt: string, maxLength: number = 50): Promise<string> {
    try {
      const response = await invoke("generate_text", {
        request: {
          prompt,
          max_length: maxLength
        }
      });
      
      return response as string;
    } catch (error) {
      console.error("Text generation error:", error);
      throw error;
    }
  }

  async function sendMessage() {
    if (!inputMessage.trim() || isLoading || !modelLoaded) return;

    const userMessage = inputMessage;
    messages = [...messages, { role: "user", content: userMessage }];
    inputMessage = "";
    isLoading = true;

    try {
      // Create a simple prompt
      const promptText = `User: ${userMessage}\nAssistant:`;

      // Generate response
      const response = await generateText(promptText, 100);

      messages = [...messages, { role: "assistant", content: response }];
    } catch (e) {
      console.error("AI query failed:", e);
      messages = [
        ...messages,
        {
          role: "assistant",
          content: `Sorry, I couldn't process that request: ${e.message}`,
        },
      ];
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="chat-container">
  <div class="messages">
    {#each messages as message}
      <div class="message {message.role}">
        <span class="role"
          >{message.role === "user" ? "You" : "Assistant"}:</span
        >
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
      on:keydown={(e) => e.key === "Enter" && sendMessage()}
      disabled={!modelLoaded || isLoading}
    />
    <button
      on:click={sendMessage}
      disabled={!modelLoaded || isLoading}
      class:loading={isLoading}
    >
      {isLoading ? "Sending..." : "Send"}
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
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
