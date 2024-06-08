<script lang="ts">
  import NuevoUsuario from "./NuevoUsuario.svelte";

  let selectedMenu = "Login";

  const changeMenu = (menu: string) => {
    selectedMenu = menu;
  };

  let correo = "";
  let contreseña = "";
</script>

<main style="margin-top: 5vh;">
  <div class="login-container">
    {#if selectedMenu === "Login"}
      <div class="login-header">
        <img style="margin-right: 450px;" src="/logicirculo.png" alt="Logo" />
      </div>

      <div class="login-form">
        <div class="svg">
          <img src="/user.svg" alt="User Icon" />
        </div>
        <p class="title">Iniciar sesión</p>
        <div class="input-container">
          <img src="/at.svg" alt="Email Icon" />
          <input
            type="text"
            placeholder="Ingresa tu correo"
            bind:value={correo}
          />
        </div>

        <div class="input-container">
          <img src="/lock.svg" alt="Password Icon" />
          <input
            type="password"
            placeholder="Contraseña"
            bind:value={contreseña}
          />
        </div>

        <button type="button">Iniciar sesión</button>

        <a
          href="#"
          class="registro"
          on:click={() => changeMenu("NuevoUsuario")}
        >
          ¿No tienes cuenta? Regístrate
        </a>
      </div>
    {/if}

    {#if selectedMenu === "NuevoUsuario"}
      <NuevoUsuario on:closeLogin={() => changeMenu("NuevoUsuario")} />
    {/if}
  </div>
</main>

<style>
  .login-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 90%;
    max-width: 600px;
    margin: 8% auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 5vh;
    border-radius: 5vh;
    background-image: url("bb.svg");
  }

  .login-header {
    display: flex;
    justify-content: center;
    margin-bottom: 20px;
  }

  .login-header img {
    max-width: 100px;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .title {
    color: #edf4f8;
    font-family: "Inria Sans", sans-serif;
    margin: 10px 0;
    font-size: 2.5vh;
  }

  .svg {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 95%;
    max-width: 100%;
    margin-top: 4vh;
  }

  .input-container {
    display: flex;
    align-items: center;
    width: 100%;
    margin-bottom: 3vh;
    border-radius: 15vh;
    border: 1px solid #094067;
    background-color: #edf4f8;
    max-width: 100%;
  }

  .input-container img {
    max-width: 100%;
    height: auto;
    margin-left: 2vh;
  }

  input[type="text"],
  input[type="password"] {
    width: 90%;
    height: 6vh;
    border: none;
    background-color: #edf4f8;
    font-family: "Inria Sans", sans-serif;
    color: var(--text);
    font-weight: bold;
    font-size: 1.5vh;
    margin-left: 1vh;
  }

  button {
    background-color: var(--red);
    border-color: var(--red);
    height: 6vh;
    width: 90%;
    border-radius: 2vh;
    color: #ffffff;
    font-size: 1.5vh;
    font-weight: bold;
    border: none;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  button:hover {
    background-color: darkred;
  }

  .registro {
    color: #edf4f8;
    font-size: 1.5vh;
    margin-top: 10px;
    text-decoration: none;
    cursor: pointer;
  }

  @media (max-width: 600px) {
    .login-container {
      padding: 3vh;
    }

    .title {
      font-size: 2vh;
    }

    input[type="text"],
    input[type="password"] {
      font-size: 1.2vh;
      height: 5vh;
    }

    button {
      font-size: 1.2vh;
      height: 5vh;
    }

    .registro {
      font-size: 1.2vh;
    }
  }
</style>
