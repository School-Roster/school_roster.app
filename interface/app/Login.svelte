<script>
  let correo = "";
  let contreseña = "";
  let value;
  function disableScroll() {
    document.body.classList.add("no-scroll");
  }
</script>

<main>
  <div class="prueba">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" alt="" />
      </div>

      <div
        style="display: flex; width:100%; justify-content:flex-start; height:100% "
      >
        <img
          style="width:45px; align-self:end"
          src="./public/antes.svg"
          alt=""
        />
      </div>
    </div>

    <div class="card">
      <div class="svg">
        <img src="/user.svg" style="width:2000px; height:200px;" alt="" />
      </div>
      <p
        style="color: #edf4f8;
    width: 95%;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 35px;"
      >
        Iniciar sesion
      </p>
      <div style="margin-top: 60px;">
        <input
          type="text"
          placeholder="Ingresa tu correo"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={correo}
        />

        <input
          type="text"
          placeholder="Contreeña"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={contreseña}
        />
        <button type="button">Guardar</button>
      </div>
      <a style="color:aliceblue" href="#">¿No tienes cuenta? Registrate</a>
    </div>
  </div>
</main>

<style>
  .prueba {
    display: flex;
    flex-direction: row;
    width: 45%;
    height: 700px;
    margin: 100px auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 50px;
    border-radius: 5rem;
    background-image: url("bb.svg");
  }

  input[type="text"] {
    border-radius: 10px;
    margin-bottom: 15px;
    background-color: #edf4f8;
    border-color: #edf4f8;
    height: 3rem;
    width: 35rem;
    border: none;
    font-weight: bold;
    font-size: 16px;
    text-indent: 30px;
  }

  button {
    background-color: #ef4565;
    border-color: #ef4565;
    height: 3rem;
    width: 18rem;
    border-radius: 10px;
    color: #ffffff;
    font-size: 1rem;
    font-family:
      system-ui,
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      Roboto,
      Oxygen,
      Ubuntu,
      Cantarell,
      "Open Sans",
      "Helvetica Neue",
      sans-serif;

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
  p {
  }
</style>
