<script lang="ts">
  import NuevoUsuario from "./NuevoUsuario.svelte";

  let selectedMenu = "Login";

  const changeMenu = (menu: string) => {
    selectedMenu = menu;
  };

  let Nombres = "";
  let ApellidoP = "";
  let ApellidoM = "";
  let Correo = "";
  let Contraseña = "";
</script>

<main style="margin-top: 5vh;">
  <div class="login-container">
    {#if selectedMenu === "Login"}
      <div style="margin-right: 450px;" class="login-image">
        <img src="/logicirculo.png" alt="" />
      </div>

      <div class="login-form">
        <p class="title">Registro</p>
        <div class="input-container">
          <img src="/user.svg" alt="User Icon" />
          <input
            type="text"
            placeholder="Nombre(s)"
            style="font-size: 16px;"
            bind:value={Nombres}
          />
        </div>

        <div class="input-container">
          <img src="/user.svg" alt="User Icon" />
          <input
            type="text"
            placeholder="Apellido paterno"
            style="font-size: 16px;"
            bind:value={ApellidoP}
          />
        </div>

        <div class="input-container">
          <img src="/user.svg" alt="User Icon" />
          <input
            type="text"
            placeholder="Apellido materno"
            style="font-size: 16px;"
            bind:value={ApellidoM}
          />
        </div>

        <div class="input-container">
          <img src="/at.svg" alt="Email Icon" />
          <input
            type="text"
            placeholder="Correo electrónico"
            style="font-size: 16px;"
            bind:value={Correo}
          />
        </div>

        <div class="input-container">
          <img src="/lock.svg" alt="Lock Icon" />
          <input
            type="password"
            placeholder="Contraseña"
            style="font-size: 16px;"
            bind:value={Contraseña}
          />
        </div>

        <button type="button">Registrar usuario</button>

        <a
          href="/"
          class="registro"
          on:click={() => changeMenu("NuevoUsuario")}
        >
          ¿Ya tienes cuenta? Inicia sesión
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
    margin-top: 190px;
  }

  .login-image img {
    max-width: 100px;
    margin-bottom: 20px;
  }

  .login-form {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
  }

  .title {
    color: #edf4f8;
    font-family: "Inria Sans", sans-serif;
    margin: 10px 0;
    font-size: 2.5vh;
  }

  .input-container {
    display: flex;
    align-items: center;
    width: 100%;
    margin-bottom: 10px;
    border-radius: 25px;
    border: 1px solid #094067;
    background-color: #edf4f8;
  }

  .input-container img {
    width: 30px;
    margin-left: 10px;
  }

  input[type="text"],
  input[type="password"] {
    width: calc(100% - 40px);
    height: 40px;
    border: none;
    background-color: #edf4f8;
    font-family: "Inria Sans", sans-serif;
    color: var(--fg-color);
    font-weight: bold;
    font-size: 16px;
    padding-left: 10px;
    border-radius: 25px;
  }

  button {
    background-color: var(--red);
    border-color: var(--red);
    height: 40px;
    width: 100%;
    border-radius: 25px;
    color: #ffffff;
    font-size: 16px;
    font-weight: bold;
    border: none;
    cursor: pointer;
    transition: background-color 0.3s;
    margin-top: 10px;
  }

  button:hover {
    background-color: darkred;
  }

  .registro {
    color: #edf4f8;
    font-size: 16px;
    margin-top: 10px;
    text-decoration: none;
    cursor: pointer;
  }
</style>
