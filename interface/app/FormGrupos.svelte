<script>
  let grado = "";
  let grupo = "";
  let carrera = "";
  let turno = "";
  let cantidadAlumnos = "";
  let value;
</script>

<main>
  <div class="prueba">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="./public/logicirculo.png" />
        <h1>Grupos</h1>
      </div>
      <div
        style="display: block; height:1px; width:100%; background-color:black"
      ></div>
      <p class="titulo">Registro de Grupos</p>

      <div
        style="display: flex; width:100%; justify-content:flex-start; height:100% "
      >
        <img style="width:45px; align-self:end" src="./public/antes.svg" />
      </div>
    </div>

    <div class="card">
      <div style="display: flex; flex-direction:column; margin:auto">
        <input type="text" placeholder="Grado" bind:value={grado} />
        <input type="text" placeholder="Grupo" bind:value={grupo} />
        <input type="text" placeholder="Carrera" bind:value={carrera} />
        <input type="text" placeholder="Turno" bind:value={turno} />
        <input
          type="text"
          placeholder="cantidad de alumnos"
          bind:value={cantidadAlumnos}
        />
        <button type="button">Guardar</button>
      </div>
    </div>
  </div>
</main>

<style>
  .prueba {
    display: flex;
    flex-direction: row;
    width: 50%;
    margin: 100px auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 50px;
    border-radius: 5rem;
  }
  h1 {
    color: #094067;
    font-size: 28px;
  }

  p {
    color: #5f6c7b;
    font-weight: bold;
    font-size: 20px;
    margin-left: 50px;
  }

  .card {
    background-color: #ffffff;
    height: 600px;
    width: 70%;
    display: flex;
    flex-direction: column;
    border-color: #000000;
    border: 1px solid;
    border-radius: 20px;
  }

  input[type="text"] {
    border-radius: 20px;
    margin-bottom: 15px;
    background-color: #edf4f8;
    border-color: #edf4f8;
    height: 3rem;
    width: 35rem;
    justify-content: center;
    align-items: center;
    border: none;
    font-weight: bold;
  }

  button {
    background-color: #ef4565;
    border-color: #ef4565;
    height: 3rem;
    width: 18rem;
    border-radius: 50px;
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
</style>
