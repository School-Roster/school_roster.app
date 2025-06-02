<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import "$styles/chat.scss";

  let messages: { role: "user" | "assistant"; content: string }[] = [];
  let inputMessage: string = "";
  let isLoading: boolean = false;
  let apiKey: string = "";
  let apiKeyValid: boolean = false;
  let selectedModel: "standard" | "reasoner" = "standard";
  let errorMessage: string = "";

  async function validateApiKey() {
    if (!apiKey.trim()) {
      errorMessage =
        "Por favor inicie sesion para poder utilizar nuestro asistente IA";
      return;
    }

    try {
      isLoading = true;
      errorMessage = "";
      const response = await invoke("check_api_key", { apiKey });
      apiKeyValid = true;
      await initializeModel();
    } catch (e) {
      console.error("Fallo en validar la llave API", e);
      errorMessage = e as string;
      apiKeyValid = false;
    } finally {
      isLoading = false;
    }
  }

  async function initializeModel() {
    try {
      isLoading = true;
      errorMessage = "";

      await invoke("init_model", { apiKey });
      messages = [
        ...messages,
        {
          role: "assistant",
          // content: 'DeepSeek AI assistant connected. How can I help with your school schedule?'
          content:
            "Asistente AI conectado. En que te puedo ayudar con tu horario escolar?",
        },
      ];
    } catch (e) {
      console.error("No se pudo inicializar el modelo IA", e);
      errorMessage = e as string;
      apiKeyValid = false;
    } finally {
      isLoading = false;
    }
  }

  async function sendMessage() {
    if (!inputMessage.trim() || isLoading || !apiKeyValid) return;

    const userMessage = inputMessage;
    messages = [...messages, { role: "user", content: userMessage }];
    inputMessage = "";
    isLoading = true;
    errorMessage = "";

    try {
      // Usa el endpoint basado en la seleccion del modelo
      const endpoint =
        selectedModel === "standard" ? "query_ai" : "query_ai_reasoner";
      const response = await invoke(endpoint, { message: userMessage });
      messages = [
        ...messages,
        { role: "assistant", content: response as string },
      ];
    } catch (e) {
      console.error("AI query failed:", e);
      errorMessage = e as string;
      messages = [
        ...messages,
        {
          role: "assistant",
          content: "Lo lamentamos, encontré un error al procesar su solicitud.",
        },
      ];
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="app-container">
  {#if !apiKeyValid}
    <div class="api-key-setup">
      <h2>Conectandose a nuestra API</h2>
      <!-- TODO: Validar con el usuario -->
      <p>Ingresa tu codigo para poder continuar</p>

      <div class="input-group">
        <input
          type="password"
          bind:value={apiKey}
          placeholder="sk-..."
          on:keydown={(e) => e.key === "Enter" && validateApiKey()}
          disabled={isLoading}
        />
        <button on:click={validateApiKey} disabled={isLoading}>
          {isLoading ? "Verificando..." : "Conectado"}
        </button>
      </div>

      {#if errorMessage}
        <div class="error-message">{errorMessage}</div>
      {/if}
    </div>
  {:else}
    <div class="chat-container">
      <div class="messages">
        {#each messages as message}
          <div class="message {message.role}">
            <span class="role"
              >{message.role === "user" ? "Tu" : "Asistente"}:</span
            >
            <p>{message.content}</p>
          </div>
        {/each}
        {#if isLoading}
          <div class="loading">Pensando...</div>
        {/if}
        {#if errorMessage}
          <div class="error-message">{errorMessage}</div>
        {/if}
      </div>

      <div class="input-area">
        <input
          type="text"
          bind:value={inputMessage}
          placeholder="Preguntame acerca de tu horario..."
          on:keydown={(e) => e.key === "Enter" && sendMessage()}
          disabled={isLoading}
        />
        <button on:click={sendMessage} disabled={isLoading}> Enviar </button>
      </div>
    </div>
  {/if}
</div>
