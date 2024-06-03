<script>
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
</script>

<div class="search-bar">
  <input type="text" id="searchQuery" placeholder="Buscar Grupos" />
</div>
<div>
  <button type="button" onclick="performSearch()">Buscar</button>
</div>

{#if results.length > 0}
  <ul>
    {#each results as result}
      <li key={result.id}>{result.title}</li>
    {/each}
  </ul>
{/if}

<style>
  .search-bar {
    display: flex;
    align-items: center;
    margin-top: 10px;
    max-width: 500px;
    margin: 20px auto;
    padding: 10px;
    background-color: #f1f1f1;
    border-radius: 25px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .search-bar input[type="text"] {
    flex: 1;
    padding: 10px 15px;
    border: none;
    border-radius: 25px;
    outline: none;
    font-size: 16px;
  }

  button {
    padding: 10px 20px;
    border: none;
    border-radius: 25px;
    background-color: #094067;
    color: white;
    cursor: pointer;
    font-size: 16px;
    transition: background-color 0.3s ease;
  }

  .search-bar button:hover {
    background-color: #45a049;
  }
</style>
