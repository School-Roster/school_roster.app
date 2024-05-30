<script>
  let correo = "";
  let contreseña = "";
</script>

<main>
  <div class="login-container">
    <div style="width: 20%; display:flex; flex-direction:column; position: absolute;">
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" alt="" />
      </div>
    </div>

    <div style="width: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center;">
      <div class="svg">
        <img src="/user.svg" style="width:199px; height:200px;" alt="" />
      </div>
      <p
        style="color: #edf4f8;
                width: 100%;
                font-family: 'Inria Sans', sans-serif;
                display: flex;
                justify-content: center;
                align-items: center;
                margin-top: 0px;
                font-size: 35px;"
      >
        Iniciar sesion
      </p>
      <div style="margin-top: 20px;">
        <div class="input-container">
          <img src="/at.svg" alt="" />
          <input
            type="text"
            placeholder="Ingresa tu correo"
            style="
              font-size: 20px;
              text-indent: 30px;
            "
            bind:value={correo}
          />
        </div>

      <div class="input-container">
        <img src="/lock.svg" alt="" />
        <input
          type="password"
          placeholder="Contraseña"
          style="
            font-size: 20px;
            text-indent: 30px;
          "
          bind:value={contreseña}
        />
      </div>
        <button type="button">Iniciar sesion</button>
      </div>
      <a href=""
        style="
          color:var(--headline);
          font-size: 18px;
          display: flex;
          margin-top: 12px;
        "
      >
        ¿No tienes cuenta? Registrate
      </a>
    </div>
  </div>
</main>

<style>
  .login-container {
    display: flex;
    flex-direction: row;
    width: 45%;
    height: 70%;
    margin: 8% auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 50px;
    border-radius: 5rem;
    background-image: url("bb.svg");
  }

  .input-container {
    border-radius: 10px;
    margin-bottom: 30px;
    border: none;
    height: 3.75rem;
    width: 27rem;
    display: flex;
    align-items: center;
    background-color: #edf4f8;
  }

  .input-container img {
    width: 40px;
    height: 40px;
    margin-left: 20px;
  }

  input[type="text"],input[type="password"] {
    width: 85%;
    height: 90%;
    border: none;
    background-color: #edf4f8;
    font-family: 'Inria Sans', sans-serif;
    color: var(--text);
    font-weight: bold;
  }

  input[type="text"]:focus,input[type="password"]:focus {
    outline: none;
  }

  input[type="text"]::placeholder,input[type="password"]::placeholder {
    color: var(--text);
    font-weight: normal;
  }

  button {
    background-color: var(--red);
    border-color: var(--red);
    height: 3rem;
    width: 27rem;
    border-radius: 20px;
    color: #ffffff;
    font-size: 1rem;
    justify-content: left;
    font-weight: bold;
    border: none;
  }
  .svg {
    display: flex;
    margin-top: 60px;
    justify-content: center;
    align-items: center;
    width: 95%;
  }
</style>
