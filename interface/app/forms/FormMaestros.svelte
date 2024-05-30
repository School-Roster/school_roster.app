<script>
  let nombre = "";
  let abreviatura = "";
  let Tipo = "";
  let color = "";
</script>

<main>
  <div class="prueba">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" />
        <h1>Maestros</h1>
      </div>
      <div
        style="display: block; height:1px; width:100%; background-color:black"
      ></div>
      <p class="titulo">Registro de Maestros</p>

      <div
        style="display: flex; width:100%; justify-content:flex-start; height:100% "
      >
        <img style="width:45px; align-self:end" src="/antes.svg" />
      </div>
    </div>

    <div class="card">
      <div style="display: flex; flex-direction:column; margin:auto">
        <input
          type="text"
          placeholder="Nombre(s)"
          style="font-family: 'Inria Serif', serif; font-size: 16px; text-indent: 30px;"
          bind:value={nombre}
        />

        <input
          type="text"
          placeholder="Apellido Paterno"
          style="font-family: 'Inria Serif', serif; font-size: 16px; text-indent: 30px;"
          bind:value={abreviatura}
        />
        <input
          type="text"
          placeholder="Apellido Materno"
          style="font-family: 'Inria Serif', serif; font-size: 16px; text-indent: 30px;"
          bind:value={Tipo}
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
    width: 70%;
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
    justify-content: left;
    font-weight: bold;
    border: none;
  }
</style>
