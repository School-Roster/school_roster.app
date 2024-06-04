<script>
  const Menus = {
    principal: "principal",
    NuevoUsuario: "nuevousuario",
  };
  let correo = "";
  let contreseña = "";
  let selectecMenu = Menus.principal;
  import NuevoUsuario from "./NuevoUsuario.svelte";
</script>

<main style="margin-top: 5vh;">
  <div class="login-container">
    <div style="flex: 1;">
      <div style="display: flex; justify-content: center; margin-bottom: 20px;">
        <img
          src="/logicirculo.png"
          alt=""
          style="max-width: 100%; height: auto;"
        />
      </div>
    </div>

    <div
      style="flex: 2; display: flex; flex-direction: column; justify-content: center; align-items: center;"
    >
      <div class="svg">
        <img src="/user.svg" style="max-width: 100%; height: auto;" alt="" />
      </div>
      <p
        style="color: #edf4f8; font-family: 'Inria Sans', sans-serif; margin: 10px 0; font-size: 2.5vh;"
      >
        Iniciar sesión
      </p>
      <div style="margin-top: 20px; width: 80%;">
        <div class="input-container">
          <img src="/at.svg" alt="" style="max-width: 100%; height: auto;" />
          <input
            type="text"
            placeholder="Ingresa tu correo"
            style="font-size: 1.5vh;"
            bind:value={correo}
          />
        </div>

        <div class="input-container">
          <img src="/lock.svg" alt="" style="max-width: 100%; height: auto;" />
          <input
            type="password"
            placeholder="Contraseña"
            style="font-size: 1.5vh;"
            bind:value={contreseña}
          />
        </div>
        <button type="button" style="font-size: 1.5vh;">Iniciar sesión</button>
      </div>
      <a
        href="./"
        on:click={() => {
          selectecMenu = Menus.NuevoUsuario;
        }}
        style="color: #edf4f8; font-size: 1.5vh; margin-top: 10px;"
      >
        ¿No tienes cuenta? Regístrate
      </a>
    </div>
  </div>
</main>
<div style="width: 100%;">
  {#if selectecMenu === Menus.principal}
    <p></p>
  {:else if selectecMenu === Menus.NuevoUsuario}
    <NuevoUsuario />
  {:else}
    <p></p>
  {/if}
</div>

<style>
  .login-container {
    display: flex;
    flex-direction: row;
    width: 40%;
    max-width: 1200px;
    margin: 8% auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 5vh;
    border-radius: 5vh;
    background-image: url("bb.svg");
  }

  .input-container {
    border-radius: 1.5vh;
    margin-bottom: 3vh;
    border: none;
    height: 6vh;
    display: flex;
    align-items: center;
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
    height: 100%;
    border: none;
    background-color: #edf4f8;
    font-family: "Inria Sans", sans-serif;
    color: var(--text);
    font-weight: bold;
    font-size: 1.5vh;
  }

  input[type="text"]:focus,
  input[type="password"]:focus {
    outline: none;
  }

  input[type="text"]::placeholder,
  input[type="password"]::placeholder {
    color: var(--text);
    font-weight: normal;
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
  }
  .svg {
    display: flex;
    margin-top: 4vh;
    justify-content: center;
    align-items: center;
    width: 95%;
    max-width: 100%;
  }

  @media screen and (max-width: 768px) {
    .login-container {
      flex-direction: column;
      width: 90%;
    }
  }
</style>
