<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { afterUpdate } from "svelte";

  import "$styles/chat.scss";

  // --------------------
  //  ESTADO DEL COMPONENTE
  // --------------------
  let messages: { role: "user" | "assistant"; content: string }[] = [];
  let inputMessage = "";
  let isLoading = false;
  let apiKey = "";
  let apiKeyValid = false;
  let selectedModel: "standard" | "reasoner" = "standard";
  let errorMessage = "";

  let chatEnd: HTMLDivElement | null = null;

  // --------------------
  //  UTILIDADES
  // --------------------
 function formatMessage(content: string): string {
  return content
    .replace(/^### (.*)$/gm, "<h3>$1</h3>")
    .replace(/^## (.*)$/gm, "<h2>$1</h2>")
    .replace(/^# (.*)$/gm, "<h1>$1</h1>")
    .replace(/\n/g, "<br>")
    .replace(/\*{1,2}([^*]+)\*{1,2}/g, "<strong>$1</strong>")
    .replace(/`([^`]+)`/g, "<code>$1</code>")
    .replace(/^- (.*)$/gm, "<li>$1</li>")
    .replace(/(<li>[^]*?<\/li>)/g, "<ul>$1</ul>");
}


  afterUpdate(() => {
    chatEnd?.scrollIntoView({ behavior: "smooth" });
  });

  // --------------------
  //  LÓGICA DE NEGOCIO
  // --------------------
  async function validateApiKey() {
    if (!apiKey.trim()) {
      errorMessage =
        "Por favor inicie sesión para poder utilizar nuestro asistente IA";

      return;
    }

    try {
      isLoading = true;
      errorMessage = "";
      await invoke("check_api_key", { apiKey });
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

          content:
            "¡Hola!, Mi nombre es Roster y estoy para ayudarte con cualquier duda. ¿En qué te puedo ayudar con tu horario escolar?",

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
          content:
            "Lo lamentamos, encontré un error al procesar su solicitud.",
        },
      ];
    } finally {
      isLoading = false;
    }
  }
</script>

<!-- --------------------
     MARKUP
     -------------------- -->
<div class="app-container">
  {#if !apiKeyValid}
    <div class="api-key-setup">
      <h2>Conectándose a nuestra API</h2>
      <p>Ingresa tu código para poder continuar</p>

      <div class="input-group">
        <input
          type="password"
          bind:value={apiKey}
          placeholder="sk-..."
          on:keydown={(e) => e.key === "Enter" && validateApiKey()}
          disabled={isLoading}
        />
        <button on:click={validateApiKey} disabled={isLoading}>
          {isLoading ? "Verificando..." : "Conectar"}
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
            <span class="role">
              {#if message.role !== "user"}
                <img src="/roster.png" alt="Roster" class="roster-icon" />
              {/if}
                {message.role === "user" ? "Tú" : "Roster"}:
            </span>
            <p>{@html formatMessage(message.content)}</p>
          </div>
        {/each}

        {#if isLoading}
          <div class="loading">Pensando…</div>
        {/if}

        {#if errorMessage}
          <div class="error-message">{errorMessage}</div>
        {/if}

        <div bind:this={chatEnd}></div>
      </div>

      <div class="input-area">
        <input
          type="text"
          bind:value={inputMessage}
          placeholder="Pregúntame acerca de tu horario…"
          on:keydown={(e) => e.key === "Enter" && sendMessage()}
          disabled={isLoading}
        />
        <button on:click={sendMessage} disabled={isLoading}>
          Enviar
        </button>
      </div>
    </div>
  {/if}
</div>
