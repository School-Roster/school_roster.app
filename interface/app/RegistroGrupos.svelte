<script>
  import FormGrupos from "./FormGrupos.svelte";

  const Menus = {
    principal: "principal",
    grupos: "grupos",
  };
  let selectecMenu = Menus.principal;
  let searchQuery = "";
  let results = [];

  // Función para realizar la búsqueda
  function performSearch() {
    // Simular la obtención de datos de una API (reemplazar con su fuente de datos real)
    const mockData = [
      { id: 1, title: "Resultado 1" },
      { id: 2, title: "Resultado 2" },
      { id: 3, title: "Resultado 3" },
    ];

    results = mockData.filter((item) =>
      item.title.toLowerCase().includes(searchQuery.toLowerCase())
    );
  }

  let value = "";
</script>

<main>
  <class class="prueba">
    <div style="width: 30%; display:flex; flex-direction:column">
      <div style="display: flex; flex-direction:row">
        <img src="./public/logicirculo.png" />
        <h1>Grupos</h1>
      </div>
      <div
        style="display: block; height:1px; width:100%; background-color:black"
      ></div>
    </div>

    <div class="card">
      <class class="encabezado">
        <h3>id</h3>
        <h2>Grado</h2>
        <h2>Grupo</h2>
        <h2>Carrera</h2>
        <h2>Turno</h2>
        <h2>N/Alumnos</h2>
      </class>
      <div
        style="display: block; height:1px; margin-top: 10px; background-color:black"
      ></div>
    </div>
    <div class="nee">
      <button
        class="icons"
        style=" font-size: 100px;
    padding: 0.2rem 0.2rem;
    display-flex:end;
    flex-direction: column;
    align-items: center;
    margin-top:50px"
      >
        <img src="/nuevo.svg" class="" alt="" />
      </button>
      <button
        style=" font-size: 100px;
    padding: 0.2rem 0.2rem;
    display-flex:end;
    flex-direction: column;
    align-items: center;
    margin-top:50px"
        on:click={() => {
          selectecMenu = Menus.grupos;
        }}
      >
        <img src="/editar.svg" class="" alt="" />
      </button>
      <button
        style=" font-size: 100px;
    padding: 0.2rem 0.2rem;
    display-flex:end;
    flex-direction: column;
    align-items: center;
    margin-top:50px"
      >
        <img src="/borrar.svg" class="" alt="" />
      </button>
    </div>
    <div class="search-bar">
      <input
        type="text"
        bind:value={searchQuery}
        onkeyup={performSearch}
        placeholder="Buscar Grupos"
      />
      <button type="button" onclick={performSearch}>Buscar</button>
    </div>
  </class>
</main>
<div style="width: 100%;">
  {#if selectecMenu === Menus.principal}
    <p></p>
  {:else if selectecMenu === Menus.grupos}
    <FormGrupos />
  {:else}
    <p></p>
  {/if}
</div>
{#if results.length > 0}
  <ul>
    {#each results as result}
      <li key={result.id}>{result.title}</li>
    {/each}
  </ul>
{/if}

<style>
  .prueba {
    display: flex;
    flex-direction: row;
    width: 80%;
    margin: 50px auto;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
    padding: 70px;
    border-radius: 10px;
  }
  h1 {
    color: #094067;
    font-size: 28px;
    padding-top: 15px;
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
    width: 50%;
    padding: 300;
    display: flex;
    flex-direction: column;
    border-color: #000000;
    border: 1px solid;
    border-radius: 20px;
  }
  .encabezado {
    display: flex;
    justify-content: space-between;
    margin: 0 30px;
    margin-top: 10px;
  }
  h2 {
    color: #094067;
    border-left: 1px solid #000000;
    padding-left: 7px;
  }
  h3 {
    color: #094067;
  }
  .nee {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    height: 30px;
  }
  /* Style the search bar container */
  .search-bar {
    display: flex;
    align-items: center;
    margin-bottom: 20px;
    margin-top: 300px;
  }

  /* Style the search input field */
  input[type="text"] {
    flex: 1; /* Allow the input to expand */
    padding: 10px;
    border: 1px solid #ccc;
    border-radius: 5px;
    font-size: 16px;
  }

  /* Style the search button */
  button {
    padding: 10px 15px;
    background-color: #ffffff;
    color: #094067;
    border-color: #007bff;
    border-radius: 5px;
    cursor: pointer;
    transition: background-color 0.3s ease; /* Add a hover effect */
  }

  /* Hover effect for the search button */
  button:hover {
    background-color: #ff0763;
  }
</style>
