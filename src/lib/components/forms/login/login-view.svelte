<script lang="ts">
  let username = "";
  let password = "";
  let error = "";

  async function handleSubmit(event: Event) {
    event.preventDefault();

    const res = await fetch("/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password }),
    });

    const data = await res.json();
    if (!res.ok) {
      error = data.error;
    } else {
      window.location.href = "/dashboard";
    }
  }
</script>

<div class="login-container">
  <h2 class="login-header">
    <img class="login-icon" src="/icons/logosr-fr.png" alt="Icon" />School
    Rooster
  </h2>
  <!--   <h3>Administrador y generador de horarios optimizado.</h3>
 -->
  <form on:submit={handleSubmit} class="login-form">
    <label for="username">Correo electronico</label>
    <input id="username" type="text" bind:value={username} required />

    <label for="password">Contraseña</label>
    <input id="password" type="password" bind:value={password} required />

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <button type="submit">Ingresar</button>

    <span class="login-separator">O</span>

    <button class="google-btn" type="submit">
      <img class="google-icon" src="/icons/google-icon.svg" alt="Icon" />
      Continuar con Google
    </button>
  </form>
</div>

<style lang="scss">
  @use "../../../../styles/variables";
  .login-container {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    height: 550px;
    width: 500px;
    padding: 30px;
    background: variables.$white-hard;
    border-radius: 10px;
    box-shadow: 0px 4px 15px rgba(0, 0, 0, 0.2);
  }

  .login-header {
    text-align: center;
    font-size: 40px;
    font-weight: bold;
    color: variables.$black;
    margin-right: 80px;
  }

  .login-form {
    display: flex;
    flex-direction: column;

    label {
      color: variables.$black;
      margin: 10px 0 5px;
      margin-left: 70px;
      font-weight: bold;
    }

    input {
      padding: 10px;
      width: 350px;
      border-radius: 10px;
      border: 1px solid #ccc;
      border-radius: 5px;
      margin-left: 70px;
      font-size: 14px;
      outline: none;
      transition:
        border 0.3s ease,
        box-shadow 0.3s ease;

      &:focus {
        border: 0.1em solid variables.$primary-color; /* here configure as your needs */
        box-shadow: 0px 0px 3px rgba(94, 129, 172, 0.5);
      }
    }

    .error {
      color: red;
      text-align: center;
      margin-top: 10px;
    }

    button {
      margin-top: 1.3rem;
      padding: 12px;
      background: variables.$black;
      height: 40px;

      color: variables.$white-hard;
      border: none;
      border-radius: 10px;
      cursor: pointer;
      font-size: 16px;
      width: 373px;
      margin-left: 70px;
      transition:
        background 0.5s ease,
        transform 0.1s ease-in-out;

      &:hover {
        background: variables.$black-overlay;
        transform: scale(1.01);
      }

      &:active {
        transform: scale(0.98);
      }
    }
    .google-btn {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 373px;
      height: 40px;
      padding: 12px;
      background: #ffffff;
      color: #333;
      font-size: 15px;
      font-weight: bold;
      border: 2px solid #000000;
      border-radius: 10px;
      cursor: pointer;
      transition:
        background 0.3s ease,
        color 0.3s ease,
        transform 0.1s ease-in-out;
      margin-top: 0;

      &:hover {
        background: #ffffff;
        color: #000000;
      }

      &:active {
        transform: scale(0.95);
      }

      .google-icon {
        width: 35px;
        height: 35px;
        margin-right: 12px;
      }

      img {
        width: 20px;
        height: 20px;
        margin-right: 10px;
      }
    }
  }

  .login-icon {
    width: 100px;
    height: auto;
    margin-left: 40px;
  }
  .login-separator {
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 15px 0;
    font-weight: bold;
    color: variables.$black-overlay;
    position: relative;
    text-transform: uppercase;
  }

  .login-separator::before,
  .login-separator::after {
    content: "";
    flex: 1;
    height: 1px;
    background: #ccc;
    margin: 0 10px;
  }
</style>
