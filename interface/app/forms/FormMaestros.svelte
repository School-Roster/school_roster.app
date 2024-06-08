<script>
  import SafeBtn from "../components/SafeBtn.svelte";
  import ColorPicker from "../components/ColorPicker.svelte";
  import ScrollBar from "../components/ScrollBar.svelte";
  import SearchBar from "../components/SearchBar.svelte";
  import CbMaterias from "../components/CBMaterias.svelte";
  import Rating from "../components/Rating.svelte";

  let Nombre = "";
  let ApellidoP = "";
  let ApellidoM = "";
  let Correo = "";
  let HorasComisionadas = "";
</script>

<main>
  <div style="margin-top: 150px;" class="prueba">
    <div>
      <div style="display: flex; flex-direction:row; align-items: center;">
        <img
          src="/logicirculo.png"
          alt="logo"
          style="width: 50px; margin-right: 10px;"
        />
        <h1>Maestros</h1>
      </div>
      <div
        style="display: block; height: 1px; width: 100%; background-color: black;"
      ></div>
      <p class="titulo">Registro de Maestros</p>

      <div style="display: flex; align-items: center;">
        <img
          src="/antes.svg"
          alt="antes"
          style="width: 45px; margin-top: 2px;"
        />
      </div>
    </div>

    <div class="card">
      <div class="placeholder" style="margin-top: 30px;">
        <input
          type="text"
          placeholder="Nombre"
          style="font-size: 16px;"
          bind:value={Nombre}
        />

        <input
          type="text"
          placeholder="Apellido Paterno"
          style="font-size: 16px;"
          bind:value={ApellidoP}
        />
        <input
          type="text"
          placeholder="Apellido Materno"
          style="font-size: 16px;"
          bind:value={ApellidoM}
        />
        <input
          type="text"
          placeholder="Correo Electronico"
          style="font-size: 16px;"
          bind:value={Correo}
        />
        <input
          type="text"
          placeholder="Horas Comisionadas"
          style="font-size: 16px;"
          bind:value={HorasComisionadas}
        />
      </div>
      <div style="margin-left: 30px; margin-top:10px;">
        <CbMaterias />
      </div>
      <div style="margin-top: 10px; margin-left: 20px; width: 100%;">
        <ColorPicker />
      </div>
      <div style="margin-left: 30px; margin-top:10px;">
        <Rating />
      </div>

      <div style="margin-top: 20px; margin-left: 270px;">
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
    margin-top: 15px;
  }

  p.titulo {
    color: #5f6c7b;
    font-weight: bold;
    font-size: 20px;
    margin-left: 50px;
    margin-bottom: 20px;
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
    width: calc(100% - 50px);
    border-color: #094067;
    margin-left: 25px;
    font-size: 16px;
    padding-left: 15px;
  }
</style>
