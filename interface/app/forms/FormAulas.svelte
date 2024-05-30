<script>
  let NumeroSalon = "";
  let Edificio = "";
  let Tipo = "";
  let Capacidad = "";
  let value;
</script>

<main>
  <div class="prueba">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" alt="" />
        <h1>Aulas</h1>
      </div>
      <div
        style="display: block; height:1px; width:100%; background-color:black"
      ></div>
      <p class="titulo">Registro de Aulas</p>

      <div
        style="display: flex; width:100%; justify-content:flex-start; height:100% "
      >
        <img style="width:45px; align-self:end" src="/antes.svg" alt="" />
      </div>
    </div>

    <div class="card">
      <div style="display: flex; flex-direction:column; margin:auto">
        <input
          type="text"
          placeholder="Numero de salon"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={NumeroSalon}
        />
        <input
          type="text"
          placeholder="Edificio"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={Edificio}
        />
        <input
          type="text"
          placeholder="Tipo"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={Tipo}
        />
        <input
          type="text"
          placeholder="Capacidad"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={Capacidad}
        />
        <button type="button">Guardar</button>
      </div>
    </div>
  </div>
</main>
¿

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
    justify-content: left;
    font-weight: bold;
    border: none;
  }
</style>
