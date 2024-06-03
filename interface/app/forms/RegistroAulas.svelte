<script>
  import SearchBar from "../components/SearchBar.svelte";
  import NuEdEl from "../components/NuEdEl.svelte";
</script>

<main style="margin-top: 140px;">
  <div class="prueba">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="/logicirculo.png" alt="" />
        <h1>Grupos</h1>
      </div>
      <div
        style="display: block; height:1px; width:150%; background-color:black"
      ></div>
      <div>
        <SearchBar />
      </div>
      <div
        style="display: flex; width:90%; justify-content:flex-start; height:100% "
      >
        <img style="width:35px; align-self:end" src="/antes.svg" alt="" />
      </div>
    </div>

    <div style="margin-left: 10px;" class="card">
      <div
        style="display:flex; gap:25px; margin-left:20px;  "
        class="parametros"
      >
        <p>id</p>
        <p>Grado</p>
        <p>Grupo</p>
        <p>Carrera</p>
        <p>Turno</p>
        <p>#Alumnos</p>
      </div>
      <div>
        <NuEdEl />
      </div>
    </div>
  </div>
</main>

<style>
  .prueba {
    display: flex;
    flex-direction: row;
    width: 60%;
    margin: 100px auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 50px;
    border-radius: 5rem;
  }
  h1 {
    color: var(--azulp);
    font-size: 28px;
  }

  .card {
    background-color: #ffffff;
    height: 600px;
    width: 50%;
    display: flex;
    flex-direction: column;
    border-color: #000000;
    border: 1px solid;
    border-radius: 20px;
    margin-left: 1px;
  }
</style>
