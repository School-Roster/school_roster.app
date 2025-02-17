<script lang="ts">
    let username = "";
    let password = "";
    let error = "";
  
    async function handleSubmit(event: Event) {
      event.preventDefault();
      
      const res = await fetch("/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password })
      });
  
      const data = await res.json();
      if (!res.ok) {
        error = data.error;
      } else {
        window.location.href = "/dashboard"; // Redirigir tras el login
      }
    }
  </script>

<div class="container">
  <h2>School Rooster</h2>
  <form on:submit={handleSubmit}>
    <label for="username">Usuario</label>
    <input id="username" type="text" bind:value={username} required />

    <label for="password">Contraseña</label>
    <input id="password" type="password" bind:value={password} required />

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <button type="submit">Ingresar</button>

    <p class="message">Inicia sesion con <a href="#"> Google </a>
  </form>
</div>

  
<style lang="scss">
  .container {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 500px;  // Aumenté el tamaño
    padding: 30px;     // Más espacio interno
    background: #fff;
    border-radius: 10px;
    box-shadow: 0px 4px 15px rgba(0, 0, 0, 0.2); // Sombra más pronunciada

    h2 {
      text-align: center;
    }
    
    form {
      display: flex;
      flex-direction: column;
      
      label {
        margin: 10px 0 5px;
      }
  
      input {
        padding: 10px;
        width: 478px;
        border: 1px solid #ccc;
        border-radius: 5px;
      }
  
      .error {
        color: red;
        text-align: center;
        margin-top: 10px;
      }
  
      button {
        margin-top: 15px;
        padding: 12px;
        background: #007bff;
        color: #fff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        font-size: 16px;
        
  
        &:hover {
          background: #0056b3;
        }
        
      }
    }
    .message {
  text-align: center;
  margin-top: 15px;
  font-size: 16px;
  color: #333; 

  a {
    color: #ff1e00; 
    text-decoration: none;
    font-weight: bold;

    &:hover {
      color: #0056b3;
      text-decoration: underline;
    }
  }
}

    
  }

  // Opcional: Fondo para ocupar toda la pantalla
  body {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh; 
    background: #f0f2f5;
  }

  
</style>
