<script>
  let nombre = "";
  let abreviatura = "";
  let Tipo = "";
  let color = "";
  let config = {
    readOnly: false,
    countStars: 5,
    range: { min: 0, max: 5, step: 0.001 },
    score: 3.785,
    showScore: true,
    scoreFormat: function () {
      return `(${this.score.toFixed(0)}/${this.countStars})`;
    },
    starConfig: {
      size: 30,
      fillColor: "#F9ED4F",
      strokeColor: "#000000",
      unfilledColor: "#FFFFFF",
      strokeUnfilledColor: "#000000F",
    },
  };

  const changeSliderInput = () => console.table(config);
</script>

<main>
  <div class="prueba">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="./public/logicirculo.png" />
        <h1>Materias</h1>
      </div>
      <div
        style="display: block; height:1px; width:100%; background-color:black"
      ></div>
      <p class="titulo">Registro de Materias</p>

      <div
        style="display: flex; width:100%; justify-content:flex-start; height:100% "
      >
        <img style="width:45px; align-self:end" src="./public/antes.svg" />
      </div>
    </div>

    <div class="card">
      <div style="display: flex; flex-direction:column; margin:auto">
        <input
          type="text"
          placeholder="Nombre"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={nombre}
        />

        <input
          type="text"
          placeholder="Abreviatura"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={abreviatura}
        />
        <input
          type="text"
          placeholder="Tipo"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={Tipo}
        />

        <div class="color-group">
          <label class="label-input" for="fill-color">Selecciona color</label>
          <input
            type="color"
            name="fill-color"
            bind:value={config.starConfig.fillColor}
            step="1"
            min="0"
          />
          <div>
            <button type="button">Guardar</button>
          </div>
        </div>
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
    color: var(--azulp);
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
