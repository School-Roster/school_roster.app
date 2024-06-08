<script>
  import SafeBtn from "../components/SafeBtn.svelte";

  let Grado = "";
  let Grupo = "";
  let Carrera = "";
  let Turno = "";
  let CantidadAlumnos;
</script>

<main>
  <div style="margin-top: 150px;" class="prueba">
    <div>
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" alt="logo" />
        <h1 style="margin-top: 15px;">Grupos</h1>
      </div>
      <div
        style="display: block; height:1px; width:100%; background-color:black"
      ></div>
      <p class="titulo">Registro de Grupos</p>

      <div
        style="display: flex; width:100%; justify-content:flex-start; height:100%; margin-top:100px "
      >
        <img
          style="width:45px; align-self:end margin-top:2px"
          src="/antes.svg"
          alt="antes"
        />
      </div>
    </div>

    <div class="card">
      <div class="placeholder" style="margin-top: 50px;">
        <input
          type="text"
          placeholder="Grado"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={Grado}
        />

        <input
          type="text"
          placeholder="Grupo"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={Grupo}
        />
        <input
          type="text"
          placeholder="Carrera"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={Carrera}
        />
        <input
          type="text"
          placeholder="Turno"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={Turno}
        />
        <input
          type="text"
          placeholder="Cantidad de alumnos"
          style="font-size: 16px; text-indent: 30px;"
          bind:value={CantidadAlumnos}
        />
      </div>

      <div style="margin-top: 200px; margin-left:270px">
        <SafeBtn />
      </div>
    </div>
  </div>
</main>

<style>
  .prueba {
    display: flex;
    flex-direction: row;
    width: 50%;
    height: 700px;
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
    height: 100%;
    width: 70%;
    display: flex;
    flex-direction: column;
    border-color: #000000;
    border: 1px solid;
    border-radius: 20px;
  }

  input[type="text"] {
    border-radius: 20px;
    margin-bottom: 10px;
    background-color: #edf4f8;
    height: 3rem;
    width: 350px;
    justify-content: center;
    align-items: center;
    border-color: #094067;

    margin-left: 25px;
  }
</style>
